//! F1c: rebuildable overlay projection + AR control-account agreement.
//!
//! `project_settlement_state` materializes the F1b derivation into the
//! `customer_invoices` overlay: one row per posted customer document with
//! the derived status, outstanding cache, terms and dates. The projection
//! is IDEMPOTENT (re-running with unchanged facts changes nothing) and
//! LOSSLESS (manual overlay states — WRITTEN_OFF / DISPUTED — are never
//! overwritten; reads already honour them through `derive_status`).
//! Truth stays in the posted facts: the overlay is a cache-like index,
//! and every value it stores equals what `ledger::compose_item` derives
//! from the same facts, which the agreement suite asserts live.
//!
//! `ar_control_drift` is the balance-sheet cross-check: the AR control
//! account's journal-line movement must equal the signed total of posted
//! customer documents (invoices and debit notes positive, credit notes
//! negative). A non-zero drift means postings bypassed the controlled
//! path or the overlay scope is wrong.
#![allow(dead_code)]
// The runtime consumer arrives with F3 (aging snapshots read the overlay);
// until then the live acceptance test in this module is the caller, matching
// the F1 pattern where derivation is proven by tests before ops expose it.

use chrono::{NaiveDate, Utc};
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rerp_entities::accounting::foundation::account::{
    Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::journal_line::{
    Column as LineColumn, Entity as LineEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    AccountingPostedDocumentModel, Column as DocumentColumn, Entity as DocumentEntity,
};
use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

use crate::ledger::{self, ArError, ItemFacts};
use rerp_accounting_accounts_receivable::models::ar_payment_application::{
    Column as ApplicationColumn, Entity as ApplicationEntity,
};
use rerp_accounting_accounts_receivable::models::customer_invoice::{
    CustomerInvoiceModel, CustomerInvoiceRecord,
};

/// Overlay states set by AR actions the postings cannot know about. The
/// projection must preserve them verbatim.
const MANUAL_STATES: [&str; 2] = ["WRITTEN_OFF", "DISPUTED"];

/// The control role of the AR receivable account (invoice posting rule).
const AR_ROLE: &str = "ACCOUNTS_RECEIVABLE";

const MAX_JOURNAL_LINES: i64 = 100_000;
const MAX_CREDIT_NOTE_DOCS: i64 = 50_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionReport {
    /// Posted customer documents scanned (bounded by the F1b read guard).
    pub documents: usize,
    /// Overlay rows created.
    pub created: u32,
    /// Overlay rows whose cached values drifted from the derivation and
    /// were rewritten.
    pub refreshed: u32,
    /// Overlay rows in a manual state, left untouched.
    pub preserved_manual: u32,
    /// AR control account minus signed document totals. `None` when the
    /// legal entity has no AR control account configured or the line
    /// volume exceeds the read bound (the check degrades, the projection
    /// never fails on reconciliation volume).
    pub ar_control_drift: Option<Decimal>,
}

fn money_of(item: &Value, key: &str) -> Decimal {
    Decimal::from_str(item[key].as_str().unwrap_or_default()).unwrap_or(Decimal::ZERO)
}

fn text_of<'a>(item: &'a Value, key: &str) -> &'a str {
    item[key].as_str().unwrap_or_default()
}

fn derived_row(
    document: &AccountingPostedDocumentModel,
    item: &Value,
) -> (Decimal, String, String, Decimal, NaiveDate) {
    (
        money_of(item, "outstanding_amount"),
        text_of(item, "status").to_string(),
        text_of(item, "terms").to_string(),
        money_of(item, "original_amount"),
        document.due_date,
    )
}

fn manual_state(row: &CustomerInvoiceModel) -> bool {
    MANUAL_STATES.contains(&row.status.as_str())
}

/// Derive the AR control-account drift for one legal entity: journal-line
/// movement on the AR control account minus the signed totals of posted
/// customer documents. Returns `Ok(None)` when the comparison is not
/// computable within read bounds (no AR account configured, or more
/// journal lines / credit-note documents than the bounds allow).
pub fn ar_control_drift<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
) -> Result<Option<Decimal>, ArError> {
    let account = AccountEntity::find()
        .filter(AccountColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(AccountColumn::LegalEntityId.eq(legal_entity))
        .filter(AccountColumn::ControlRole.eq(AR_ROLE.to_string()))
        .filter(AccountColumn::IsActive.eq(true))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let Some(account) = account else {
        return Ok(None);
    };

    // Bounded journal-line sweep on the control account.
    let lines = LineEntity::find()
        .filter(LineColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(LineColumn::LegalEntityId.eq(legal_entity))
        .filter(LineColumn::AccountId.eq(account.id))
        .limit((MAX_JOURNAL_LINES + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if lines.len() as i64 > MAX_JOURNAL_LINES {
        return Ok(None);
    }
    let journal_net = lines.iter().fold(Decimal::ZERO, |acc, line| {
        if line.side == "DEBIT" {
            acc + line.amount
        } else {
            acc - line.amount
        }
    });

    // Signed document totals: chargebacks positive, credit notes negative.
    let credit_notes = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.eq("CUSTOMER_CREDIT_NOTE".to_string()))
        .limit((MAX_CREDIT_NOTE_DOCS + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if credit_notes.len() as i64 > MAX_CREDIT_NOTE_DOCS {
        return Ok(None);
    }
    let documents = ledger::chargeback_documents(executor, session, &[legal_entity], None)?;
    let chargeback_total: Decimal = documents.iter().map(|doc| doc.total_amount).sum();
    let credit_total: Decimal = credit_notes.iter().map(|doc| doc.total_amount).sum();
    let expected = chargeback_total - credit_total;
    Ok(Some(journal_net - expected))
}

/// Serialize overlay projections per legal entity with a
/// transaction-scoped advisory lock — the same FNV pattern as GL's
/// posting lock. Concurrent runs for one entity must not both take the
/// create path for the same document and abort on
/// `customer_invoices_document_unique`.
fn overlay_projection_lock<E: LifeExecutor>(
    executor: &E,
    legal_entity: Uuid,
) -> Result<(), ArError> {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in b"ar-overlay-projection"
        .iter()
        .chain(legal_entity.as_bytes().iter())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    let lock_key = i64::from_be_bytes(hash.to_be_bytes());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| ArError::Database(format!("overlay projection lock: {error}")))?;
    Ok(())
}

/// Rebuild the overlay for one legal entity from the posted facts.
/// Idempotent; manual WRITTEN_OFF / DISPUTED states are preserved.
pub fn project_settlement_state<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
) -> Result<ProjectionReport, ArError> {
    overlay_projection_lock(executor, legal_entity)?;
    let today = Utc::now().date_naive();
    let documents = ledger::chargeback_documents(executor, session, &[legal_entity], None)?;
    let document_ids: Vec<Uuid> = documents.iter().map(|doc| doc.id).collect();
    let credits =
        ledger::credit_note_totals(executor, session, &[legal_entity], document_ids.clone())?;
    let overlays = ledger::overlay_state(executor, session, &[legal_entity], document_ids)?;

    let mut report = ProjectionReport {
        documents: documents.len(),
        created: 0,
        refreshed: 0,
        preserved_manual: 0,
        ar_control_drift: None,
    };

    for document in &documents {
        let (overlay, applied) = match overlays.get(&document.id) {
            Some((row, applied)) => (Some(row), *applied),
            None => (None, Decimal::ZERO),
        };
        if overlay.is_some_and(manual_state) {
            report.preserved_manual += 1;
            continue;
        }
        let facts = ItemFacts {
            document,
            credit_notes: credits.get(&document.id).copied().unwrap_or(Decimal::ZERO),
            overlay,
            applied,
            today,
        };
        let item = ledger::compose_item(&facts);
        let (outstanding, status, terms, original, due_date) = derived_row(document, &item);

        match overlay {
            None => {
                let mut record = CustomerInvoiceRecord::new();
                record
                    .set_id(Uuid::new_v4())
                    .set_tenant_id(session.tenant_id.clone())
                    .set_legal_entity_id(legal_entity)
                    .set_invoice_id(document.id)
                    .set_customer_id(document.customer_id)
                    // Cached original_amount is the CREDIT-NETTED original,
                    // exactly what compose_item reports on the read path.
                    .set_original_amount(original)
                    .set_outstanding_amount(outstanding)
                    .set_currency_code(document.currency_code.clone())
                    .set_due_date(due_date)
                    .set_terms(terms)
                    .set_status(status)
                    .set_created_at(Utc::now().naive_utc())
                    .set_updated_at(Utc::now().naive_utc());
                record
                    .insert(executor)
                    .map_err(|error| ArError::Database(error.to_string()))?;
                report.created += 1;
            }
            Some(row) => {
                let drifted = row.outstanding_amount != outstanding
                    || row.status != status
                    || row.terms != terms
                    || row.original_amount != original;
                if drifted {
                    let mut changes = CustomerInvoiceRecord::new();
                    changes
                        .set_id(row.id)
                        .set_outstanding_amount(outstanding)
                        .set_status(status)
                        .set_terms(terms)
                        .set_original_amount(original)
                        .set_updated_at(Utc::now().naive_utc());
                    changes
                        .update(executor)
                        .map_err(|error| ArError::Database(error.to_string()))?;
                    report.refreshed += 1;
                }
            }
        }
    }

    report.ar_control_drift = ar_control_drift(executor, session, legal_entity)?;
    Ok(report)
}

/// Latest-application stamp: (date, amount) of the most recent journal-less
/// application on an overlay row; the F2b cache contract for
/// `last_payment_date` / `last_payment_amount`.
fn last_payment_stamp<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    overlay_row_id: Uuid,
) -> Result<(Option<NaiveDate>, Option<Decimal>), ArError> {
    let rows = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::InvoiceId.eq(overlay_row_id))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    // Only ACTIVE allocations stamp the cache: reversal keeps the pair
    // row (applied_amount = 0) with its original applied_at, and a
    // reversed row must not masquerade as the document's last payment.
    let latest = rows
        .into_iter()
        .filter(|row| row.applied_amount > Decimal::ZERO)
        .max_by_key(|row| (row.applied_at, row.id));
    Ok(latest.map_or((None, None), |row| {
        (Some(row.applied_at.date()), Some(row.applied_amount))
    }))
}

/// Refresh ONE document's overlay row with the bulk projection's
/// create/refresh semantics; manual states stay preserved (F1c rule).
/// The F2b application flow calls this after mutating application facts,
/// so cache maintenance always routes through the same derivation instead
/// of hand-patched columns.
pub(crate) fn refresh_overlay_row<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
    document: &AccountingPostedDocumentModel,
) -> Result<(), ArError> {
    let document_ids = vec![document.id];
    let credits =
        ledger::credit_note_totals(executor, session, &[legal_entity], document_ids.clone())?;
    let overlays = ledger::overlay_state(executor, session, &[legal_entity], document_ids)?;
    let (overlay, applied) = match overlays.get(&document.id) {
        Some((row, applied)) => (Some(row), *applied),
        None => (None, Decimal::ZERO),
    };
    if overlay.is_some_and(manual_state) {
        return Ok(());
    }
    let stamp = overlay
        .map(|row| last_payment_stamp(executor, session, row.id))
        .transpose()?;
    let facts = ItemFacts {
        document,
        credit_notes: credits.get(&document.id).copied().unwrap_or(Decimal::ZERO),
        overlay,
        applied,
        today: Utc::now().date_naive(),
    };
    let item = ledger::compose_item(&facts);
    let (outstanding, status, terms, original, due_date) = derived_row(document, &item);
    match overlay {
        None => {
            let mut record = CustomerInvoiceRecord::new();
            record
                .set_id(Uuid::new_v4())
                .set_tenant_id(session.tenant_id.clone())
                .set_legal_entity_id(legal_entity)
                .set_invoice_id(document.id)
                .set_customer_id(document.customer_id)
                .set_original_amount(original)
                .set_outstanding_amount(outstanding)
                .set_currency_code(document.currency_code.clone())
                .set_due_date(due_date)
                .set_terms(terms)
                .set_status(status)
                .set_created_at(Utc::now().naive_utc())
                .set_updated_at(Utc::now().naive_utc());
            if let Some((stamp_date, stamp_amount)) = stamp {
                record
                    .set_last_payment_date(stamp_date)
                    .set_last_payment_amount(stamp_amount);
            }
            record
                .insert(executor)
                .map_err(|error| ArError::Database(error.to_string()))?;
        }
        Some(row) => {
            let (stamp_date, stamp_amount) = stamp.unwrap_or((None, None));
            let drifted = row.outstanding_amount != outstanding
                || row.status != status
                || row.terms != terms
                || row.original_amount != original
                || row.last_payment_date != stamp_date
                || row.last_payment_amount != stamp_amount;
            if drifted {
                let mut changes = CustomerInvoiceRecord::new();
                changes
                    .set_id(row.id)
                    .set_outstanding_amount(outstanding)
                    .set_status(status)
                    .set_terms(terms)
                    .set_original_amount(original)
                    .set_last_payment_date(stamp_date)
                    .set_last_payment_amount(stamp_amount)
                    .set_updated_at(Utc::now().naive_utc());
                changes
                    .update(executor)
                    .map_err(|error| ArError::Database(error.to_string()))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_support::with_ar_transaction;
    use crate::ledger::{list_open_items, parse_list_filters};
    use chrono::TimeDelta;
    use serde_json::json;

    #[test]
    fn manual_states_are_recognised() {
        // The preservation rule keys off exactly these two states.
        assert!(MANUAL_STATES.contains(&"WRITTEN_OFF"));
        assert!(MANUAL_STATES.contains(&"DISPUTED"));
        assert!(!MANUAL_STATES.contains(&"PARTIAL"));
        assert!(!MANUAL_STATES.contains(&"PAID"));
    }

    #[test]
    fn money_and_text_readers_tolerate_missing_keys() {
        let item = serde_json::json!({"status": "PARTIAL"});
        assert_eq!(money_of(&item, "outstanding_amount"), Decimal::ZERO);
        assert_eq!(text_of(&item, "terms"), "");
        assert_eq!(text_of(&item, "status"), "PARTIAL");
    }

    fn run_hex() -> String {
        static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        TAG.get_or_init(|| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            // Salted against the ledger suite's tag: both tests share one
            // binary, and the run tags (hence tenants and ids) must differ.
            let mix = (now ^ ((std::process::id() as u64) << 32))
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ 0xA5A5_A5A5;
            let mix = (mix >> 24) & 0x00FF_FFFF;
            format!("{mix:06x}")
        })
        .clone()
    }

    fn uid(slot: &str, run: &str) -> String {
        format!("{slot}{run}-0000-4000-8000-000000000000")
    }

    fn psql_env(cmd: &mut std::process::Command) {
        cmd.env(
            "PGHOST",
            std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into()),
        )
        .env(
            "PGPORT",
            std::env::var("DB_PORT").unwrap_or_else(|_| "5432".into()),
        )
        .env(
            "PGUSER",
            std::env::var("DB_USER").unwrap_or_else(|_| "rerp".into()),
        )
        .env(
            "PGPASSWORD",
            std::env::var("DB_PASS").unwrap_or_else(|_| "".into()),
        )
        .env(
            "PGDATABASE",
            std::env::var("DB_NAME").unwrap_or_else(|_| "rerp".into()),
        )
        .args(["-v", "ON_ERROR_STOP=1", "-q", "-At"]);
    }

    /// Seeds via STDIN (CodeQL: never write constructed SQL text to a temp
    /// file — that counts as logging sensitive data), same session shape as
    /// the ledger suite: the GUC lives inside the seed transaction.
    fn psql_run(sql: &str) -> (bool, String, String) {
        use std::io::Write;
        use std::process::Stdio;
        let mut cmd = std::process::Command::new("psql");
        psql_env(&mut cmd);
        cmd.stdin(Stdio::piped());
        // wait_with_output() only captures what was explicitly piped —
        // without these, result rows inherit the parent's stdout instead.
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn().expect("psql on PATH");
        child
            .stdin
            .as_mut()
            .expect("piped stdin")
            .write_all(sql.as_bytes())
            .expect("pipe sql");
        let output = child.wait_with_output().expect("psql on PATH");
        (
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        )
    }

    fn psql_exec(sql: &str) {
        let (ok, _stdout, stderr) = psql_run(sql);
        assert!(ok, "seed failed: {stderr}");
    }

    #[allow(clippy::too_many_arguments)]
    fn session_guc(tenant: &str, subject: &str, org: &str, app: &str) -> String {
        format!(
            "SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
                '{app}', '[\"billing\"]'::jsonb,
                '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');"
        )
    }

    /// Reads through RLS: the tenant GUC lives in the session that runs the
    /// query, so the read runs inside the same psql session.
    fn psql_session_query(session_sql: &str, body_sql: &str) -> String {
        let sql = format!("BEGIN;\n{session_sql}\n{body_sql};\nCOMMIT;");
        let (ok, stdout, stderr) = psql_run(&sql);
        assert!(ok, "query failed: {stderr}");
        stdout.trim().to_string()
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0006) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_projection_rebuilds_and_agrees_with_control_account() {
        // Tenant A holds four posted customer documents:
        //   inv1 untouched              -> created  CONFIRMED  outstanding 100
        //   inv2 minus credit note 20   -> created  PARTIAL    outstanding 40
        //   inv3 minus credit note 10   -> manual WRITTEN_OFF row must be
        //                                  preserved verbatim (frozen cache)
        //   inv4 with a STALE overlay   -> refreshed (terms NET_15 -> NET_30)
        //   inv5 with a DISPUTED overlay-> preserved (manual_state must gate
        //                                 the refresh branch, not just create)
        // Balanced journals put AR 254 on the control account, matching the
        // signed document totals; a stray DEBIT line afterwards must read as
        // drift +7. Tenant B's single invoice must stay invisible (documents
        // stays 5).
        let run = run_hex();
        let tenant = format!("ar-proj-{run}");
        let other = format!("ar-proj-x-{run}");
        let org = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let customer = Uuid::new_v4().to_string();
        let entity = uid("e0", &run);
        let period = uid("f0", &run);
        let ar_account = uid("aa", &run);
        let rev_account = uid("50", &run);
        let inv1 = uid("b1", &run);
        let inv2 = uid("b2", &run);
        let inv3 = uid("b3", &run);
        let inv4 = uid("b4", &run);
        let inv5 = uid("b5", &run);
        let cr1 = uid("c1", &run);
        let cr2 = uid("c2", &run);
        let je1 = uid("a1", &run);
        let je2 = uid("a2", &run);
        let je3 = uid("a3", &run);
        let je4 = uid("a4", &run);
        let je5 = uid("a5", &run);
        let je6 = uid("a6", &run);
        let je7 = uid("a7", &run);
        let overlay3 = uid("c3", &run);
        let overlay4 = uid("c4", &run);
        let overlay5 = uid("c5", &run);
        let b_entity = uid("b0", &run);
        let b_period = uid("bb", &run);
        let b_doc = uid("bd", &run);
        let today = Utc::now().date_naive();
        let day = |offset: i64| {
            (today + TimeDelta::days(offset))
                .format("%Y-%m-%d")
                .to_string()
        };

        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_a}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org}', 'Projection Test', 'GBP', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{period}', '{tenant}', '{entity}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_accounts
    (id, tenant_id, legal_entity_id, code, name, account_type, normal_side,
     control_role, currency_code, is_active, created_at, updated_at)
VALUES
    ('{ar_account}', '{tenant}', '{entity}', '{tenant}-AR', 'AR Control',
     'ASSET', 'DEBIT', 'ACCOUNTS_RECEIVABLE', 'GBP', true, now(), now()),
    ('{rev_account}', '{tenant}', '{entity}', '{tenant}-REV', 'Revenue',
     'REVENUE', 'CREDIT', NULL, 'GBP', true, now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by, original_document_id)
VALUES
    ('{inv1}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-1',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'one',
     '{d0}', '{d30}', 'GBP', 2, 100, 0, 0, 100.000000, now(), '{subject}'::uuid, NULL),
    ('{inv2}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-2',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'two',
     '{dm45}', '{dm15}', 'GBP', 2, 60, 0, 0, 60.000000, now(), '{subject}'::uuid, NULL),
    ('{cr1}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-1',
     'CUSTOMER_CREDIT_NOTE', '{customer}', 'proj-test', 'fixture', 'credit1',
     '{d0}', '{d0}', 'GBP', 2, 20, 0, 0, 20.000000, now(), '{subject}'::uuid, '{inv2}'),
    ('{inv3}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-3',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'three',
     '{dm30}', '{d30}', 'GBP', 2, 50, 0, 0, 50.000000, now(), '{subject}'::uuid, NULL),
    ('{cr2}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-2',
     'CUSTOMER_CREDIT_NOTE', '{customer}', 'proj-test', 'fixture', 'credit2',
     '{d0}', '{d30}', 'GBP', 2, 10, 0, 0, 10.000000, now(), '{subject}'::uuid, '{inv3}'),
    ('{inv4}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-4',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'four',
     '{dm30}', '{d0}', 'GBP', 2, 44, 0, 0, 44.000000, now(), '{subject}'::uuid, NULL),
    ('{inv5}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-5',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'five',
     '{d0}', '{d30}', 'GBP', 2, 30, 0, 0, 30.000000, now(), '{subject}'::uuid, NULL);
INSERT INTO accounting_journal_entries
    (id, tenant_id, legal_entity_id, fiscal_period_id, entry_number, entry_date,
     source_document_id, currency_code, total_debit, total_credit, posted_at, posted_by)
VALUES
    ('{je1}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-1', '{d0}',
     '{inv1}', 'GBP', 100, 100, now(), '{subject}'::uuid),
    ('{je2}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-2', '{d0}',
     '{inv2}', 'GBP', 60, 60, now(), '{subject}'::uuid),
    ('{je3}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-3', '{d0}',
     '{cr1}', 'GBP', 20, 20, now(), '{subject}'::uuid),
    ('{je4}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-4', '{d0}',
     '{inv3}', 'GBP', 50, 50, now(), '{subject}'::uuid),
    ('{je5}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-5', '{d0}',
     '{cr2}', 'GBP', 10, 10, now(), '{subject}'::uuid),
    ('{je6}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-6', '{d0}',
     '{inv4}', 'GBP', 44, 44, now(), '{subject}'::uuid),
    ('{je7}', '{tenant}', '{entity}', '{period}', '{tenant}-JE-7', '{d0}',
     '{inv5}', 'GBP', 30, 30, now(), '{subject}'::uuid);
INSERT INTO accounting_journal_lines
    (tenant_id, legal_entity_id, journal_entry_id, line_number, account_id, side,
     amount, description)
VALUES
    ('{tenant}', '{entity}', '{je1}',   1, '{ar_account}',  'DEBIT',  100, 'inv1 ar'),
    ('{tenant}', '{entity}', '{je1}',   2, '{rev_account}', 'CREDIT', 100, 'inv1 rev'),
    ('{tenant}', '{entity}', '{je2}',   1, '{ar_account}',  'DEBIT',  60,  'inv2 ar'),
    ('{tenant}', '{entity}', '{je2}',   2, '{rev_account}', 'CREDIT', 60,  'inv2 rev'),
    ('{tenant}', '{entity}', '{je3}',   1, '{rev_account}', 'DEBIT',  20,  'cr1 rev'),
    ('{tenant}', '{entity}', '{je3}',   2, '{ar_account}',  'CREDIT', 20,  'cr1 ar'),
    ('{tenant}', '{entity}', '{je4}',   1, '{ar_account}',  'DEBIT',  50,  'inv3 ar'),
    ('{tenant}', '{entity}', '{je4}',   2, '{rev_account}', 'CREDIT', 50,  'inv3 rev'),
    ('{tenant}', '{entity}', '{je5}',   1, '{rev_account}', 'DEBIT',  10,  'cr2 rev'),
    ('{tenant}', '{entity}', '{je5}',   2, '{ar_account}',  'CREDIT', 10,  'cr2 ar'),
    ('{tenant}', '{entity}', '{je6}',   1, '{ar_account}',  'DEBIT',  44,  'inv4 ar'),
    ('{tenant}', '{entity}', '{je6}',   2, '{rev_account}', 'CREDIT', 44,  'inv4 rev'),
    ('{tenant}', '{entity}', '{je7}',   1, '{ar_account}',  'DEBIT',  30,  'inv5 ar'),
    ('{tenant}', '{entity}', '{je7}',   2, '{rev_account}', 'CREDIT', 30,  'inv5 rev');
INSERT INTO customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, status, updated_at)
VALUES
    ('{overlay3}', '{tenant}', '{entity}', '{inv3}', '{customer}',
     50.000000, 50.000000, 'GBP', '{d30}', 'NET_30', 'WRITTEN_OFF', now()),
    ('{overlay4}', '{tenant}', '{entity}', '{inv4}', '{customer}',
     44.000000, 999.000000, 'GBP', '{d0}', 'NET_15', 'CONFIRMED', now()),
    ('{overlay5}', '{tenant}', '{entity}', '{inv5}', '{customer}',
     30.000000, 30.000000, 'GBP', '{d30}', 'NET_30', 'DISPUTED', now());
COMMIT;
BEGIN;
{guc_b}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{b_entity}', '{other}', '{org}', 'Projection Other', 'GBP', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{b_period}', '{other}', '{b_entity}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by)
VALUES ('{b_doc}', '{other}', '{b_entity}', '{b_period}', '{other}-INV-1',
     'CUSTOMER_INVOICE', '{customer}', 'proj-test', 'fixture', 'other',
     '{d0}', '{d30}', 'GBP', 2, 48, 0, 0, 48.000000, now(), '{subject}'::uuid);
COMMIT;",
            guc_a = session_guc(&tenant, &subject.to_string(), &org.to_string(), "ar-proj-a"),
            guc_b = session_guc(&other, &subject.to_string(), &org.to_string(), "ar-proj-b"),
            d0 = day(0),
            d30 = day(30),
            dm15 = day(-15),
            dm30 = day(-30),
            dm45 = day(-45),
        ));

        let session = SessionContext {
            tenant_id: tenant.clone(),
            subject_id: subject,
            organization_id: org,
            session_id: "ar-projection-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:ledger:read".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        };
        let entity_uuid = Uuid::parse_str(&entity).expect("entity uuid");

        // Run 1: create the missing rows, refresh the stale one, freeze the
        // manual one; the AR control account agrees with the signed totals.
        let report1 = with_ar_transaction(&session, |executor| {
            project_settlement_state(executor, &session, entity_uuid)
        })
        .expect("projection run 1");
        assert_eq!(report1.documents, 5, "tenant B's document is invisible");
        assert_eq!(report1.created, 2, "inv1 + inv2 rows created");
        assert_eq!(report1.refreshed, 1, "inv4 stale row rewritten");
        assert_eq!(
            report1.preserved_manual, 2,
            "inv3 WRITTEN_OFF and inv5 DISPUTED untouched"
        );
        assert_eq!(report1.ar_control_drift, Some(Decimal::ZERO));

        // The overlay dump is the projection's cached truth. Reads run
        // through RLS with the tenant GUC set in the same psql session.
        let dump_sql = |tenant: &str| {
            format!(
                "SELECT invoice_id || '|' || original_amount || '|' || outstanding_amount
                        || '|' || status || '|' || terms
                   FROM customer_invoices
                  WHERE tenant_id = '{tenant}' ORDER BY invoice_id"
            )
        };
        let read_guc = session_guc(
            &tenant,
            &subject.to_string(),
            &org.to_string(),
            "ar-proj-read",
        );
        let dump1 = psql_session_query(&read_guc, &dump_sql(&tenant));
        let expected_dump = format!(
            "{inv1}|100.000000|100.000000|CONFIRMED|NET_30\n\
             {inv2}|40.000000|40.000000|PARTIAL|NET_30\n\
             {inv3}|50.000000|50.000000|WRITTEN_OFF|NET_30\n\
             {inv4}|44.000000|44.000000|CONFIRMED|NET_30\n\
             {inv5}|30.000000|30.000000|DISPUTED|NET_30"
        );
        assert_eq!(dump1, expected_dump, "projection cached values");
        assert!(
            !dump1.contains("ar-proj-x"),
            "tenant B wrote no overlay rows"
        );

        // Read-path agreement: every cached value equals what the F1b
        // derivation reports over the same facts.
        let filters =
            parse_list_filters(None, None, None, None, None, None, None, None, None, None)
                .expect("default filters");
        let listed = with_ar_transaction(&session, |executor| {
            list_open_items(executor, &session, &filters)
        })
        .expect("list open items");
        assert_eq!(listed["total"], 5);
        // `label` is a static fixture name: CodeQL (cleartext-logging)
        // forbids interpolating uid()-derived ids into panic messages.
        let item = |doc: &str, label: &str| -> Value {
            listed["items"]
                .as_array()
                .expect("items array")
                .iter()
                .find(|row| row["invoice_id"] == json!(doc))
                .cloned()
                .unwrap_or_else(|| panic!("open item {label} missing"))
        };
        assert_eq!(item(&inv1, "inv1")["status"], "CONFIRMED");
        assert_eq!(item(&inv1, "inv1")["outstanding_amount"], "100");
        assert_eq!(item(&inv1, "inv1")["terms"], "NET_30");
        assert_eq!(item(&inv2, "inv2")["status"], "PARTIAL");
        assert_eq!(item(&inv2, "inv2")["outstanding_amount"], "40");
        assert_eq!(
            item(&inv2, "inv2")["original_amount"],
            "40",
            "nets the credit note"
        );
        // The read path honours the manual status but derives money from
        // the facts; the cache stays frozen at the write-off moment.
        assert_eq!(item(&inv3, "inv3")["status"], "WRITTEN_OFF");
        assert_eq!(item(&inv3, "inv3")["outstanding_amount"], "40");
        assert_eq!(item(&inv4, "inv4")["status"], "CONFIRMED");
        assert_eq!(item(&inv4, "inv4")["outstanding_amount"], "44");
        assert_eq!(
            item(&inv4, "inv4")["terms"],
            "NET_30",
            "stale NET_15 was refreshed"
        );
        assert_eq!(item(&inv5, "inv5")["status"], "DISPUTED");
        assert_eq!(item(&inv5, "inv5")["outstanding_amount"], "30");

        // Idempotence: nothing left to write.
        let report2 = with_ar_transaction(&session, |executor| {
            project_settlement_state(executor, &session, entity_uuid)
        })
        .expect("projection run 2");
        assert_eq!(report2.created, 0);
        assert_eq!(report2.refreshed, 0);
        assert_eq!(report2.preserved_manual, 2);
        assert_eq!(report2.ar_control_drift, Some(Decimal::ZERO));

        // From-scratch rebuild: drop every derived row, keep the manual one.
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_a}
DELETE FROM customer_invoices WHERE tenant_id = '{tenant}'
    AND status NOT IN ('WRITTEN_OFF', 'DISPUTED');
COMMIT;",
            guc_a = session_guc(
                &tenant,
                &subject.to_string(),
                &org.to_string(),
                "ar-proj-wipe"
            ),
        ));
        let report3 = with_ar_transaction(&session, |executor| {
            project_settlement_state(executor, &session, entity_uuid)
        })
        .expect("projection run 3");
        assert_eq!(report3.created, 3, "inv1 + inv2 + inv4 rebuilt from facts");
        assert_eq!(report3.refreshed, 0);
        assert_eq!(report3.preserved_manual, 2);
        assert_eq!(report3.ar_control_drift, Some(Decimal::ZERO));
        let dump3 = psql_session_query(&read_guc, &dump_sql(&tenant));
        assert_eq!(
            dump1, dump3,
            "from-scratch rebuild agrees with the first build"
        );

        // Control-account bypass: a stray journal line on the AR control
        // account (posting outside the controlled path) must surface as drift.
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_a}
INSERT INTO accounting_journal_lines
    (tenant_id, legal_entity_id, journal_entry_id, line_number, account_id, side,
     amount, description)
VALUES ('{tenant}', '{entity}', '{je1}', 9, '{ar_account}', 'DEBIT', 7, 'bypass probe');
COMMIT;",
            guc_a = session_guc(
                &tenant,
                &subject.to_string(),
                &org.to_string(),
                "ar-proj-drift"
            ),
        ));
        let drift = with_ar_transaction(&session, |executor| {
            ar_control_drift(executor, &session, entity_uuid)
        })
        .expect("drift recheck");
        assert_eq!(drift, Some(Decimal::from(7)), "stray AR debit is drift");
    }
}
