//! F2b: payment applications (allocations) over the rebuildable overlay.
//!
//! The contract's `customer_invoice_id` lives in the OPEN-ITEM id space —
//! the posted document id F1b echoes in list/get — while
//! `ar_payment_applications.invoice_id` FKs the overlay row. The flow
//! therefore heals the overlay row through the shared projection refresh
//! first, then allocates against the DOCUMENT-derived balance.
//!
//! The pair row (UNIQUE per payment+document) stores the accumulated
//! CURRENT allocation; every delta is journaled to
//! `ar_payment_application_events`, which is how repeated applications and
//! (F2c) reversals carry immutable audit linkage. Lock discipline: the
//! payment advisory lock is always taken before any document lock, so
//! concurrent allocations serialize predictably (no lock cycle).

use chrono::NaiveDateTime;
use chrono::Utc;
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

use crate::ledger::{self, raw_outstanding, ArError, ItemFacts};
use crate::payments::payment_json;
use crate::projection::refresh_overlay_row;
use rerp_accounting_accounts_receivable::models::ar_payment::{
    ArPaymentModel, ArPaymentRecord, Column as PaymentColumn, Entity as PaymentEntity,
};
use rerp_accounting_accounts_receivable::models::ar_payment_application::{
    ArPaymentApplicationModel, ArPaymentApplicationRecord, Column as ApplicationColumn,
    Entity as ApplicationEntity,
};
use rerp_accounting_accounts_receivable::models::ar_payment_application_event::ArPaymentApplicationEventRecord;
use rerp_accounting_accounts_receivable::models::customer_invoice::{
    Column as OverlayColumn, CustomerInvoiceModel, Entity as OverlayEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    AccountingPostedDocumentModel, Column as DocumentColumn, Entity as DocumentEntity,
};

fn money(value: Decimal) -> String {
    value.round_dp(6).normalize().to_string()
}

fn rfc3339(value: NaiveDateTime) -> String {
    value.and_utc().to_rfc3339()
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid, ArError> {
    Uuid::parse_str(value).map_err(|_| ArError::Validation(format!("{field} must be a UUID")))
}

fn parse_positive_amount(value: &str, field: &str) -> Result<Decimal, ArError> {
    let amount = Decimal::from_str(value)
        .map_err(|_| ArError::Validation(format!("{field} must be a decimal string")))?;
    if amount <= Decimal::ZERO {
        return Err(ArError::Validation(format!(
            "{field} must be greater than zero"
        )));
    }
    Ok(amount)
}

fn advisory_lock<E: LifeExecutor>(
    executor: &E,
    namespace: &[u8],
    target: Uuid,
    label: &str,
) -> Result<(), ArError> {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in namespace.iter().chain(target.as_bytes().iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    let lock_key = i64::from_be_bytes(hash.to_be_bytes());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| ArError::Database(format!("{label} lock: {error}")))?;
    Ok(())
}

fn payment_lock<E: LifeExecutor>(executor: &E, payment_id: Uuid) -> Result<(), ArError> {
    advisory_lock(
        executor,
        b"ar-payment-application",
        payment_id,
        "payment allocation",
    )
}

fn document_lock<E: LifeExecutor>(executor: &E, document_id: Uuid) -> Result<(), ArError> {
    advisory_lock(
        executor,
        b"ar-application-document",
        document_id,
        "document allocation",
    )
}

/// Contract `PaymentApplication`. `customer_invoice_id` echoes the DOCUMENT
/// id (the open-item space clients hold), matching `payment_history`.
fn application_json(row: &ArPaymentApplicationModel, document_id: Uuid, currency: &str) -> Value {
    json!({
        "id": row.id.to_string(),
        "payment_id": row.payment_id.to_string(),
        "customer_invoice_id": document_id.to_string(),
        "applied_amount": money(row.applied_amount),
        "currency_code": currency,
        "created_at": rfc3339(row.applied_at),
    })
}

fn find_payment<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment_id: &str,
) -> Result<ArPaymentModel, ArError> {
    let id = parse_uuid(payment_id, "payment_id")?;
    let scope = ledger::resolve_scope_entities(executor, session, None)?;
    PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| scope.contains(&row.legal_entity_id))
        .ok_or(ArError::NotFound)
}

/// Balance available to receive money: net outstanding BEFORE clamping,
/// clamped at zero (over-collection is a data problem, not capacity).
fn document_capacity<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    document: &AccountingPostedDocumentModel,
) -> Result<Decimal, ArError> {
    let document_ids = vec![document.id];
    let credits = ledger::credit_note_totals(
        executor,
        session,
        &[document.legal_entity_id],
        document_ids.clone(),
    )?;
    let overlays =
        ledger::overlay_state(executor, session, &[document.legal_entity_id], document_ids)?;
    let (row, applied) = overlays
        .get(&document.id)
        .map_or((None, Decimal::ZERO), |(row, applied)| {
            (Some(row), *applied)
        });
    let facts = ItemFacts {
        document,
        credit_notes: credits.get(&document.id).copied().unwrap_or(Decimal::ZERO),
        overlay: row,
        applied,
        today: Utc::now().date_naive(),
    };
    Ok(raw_outstanding(&facts).max(Decimal::ZERO))
}

/// Upsert the pair row, journal the event. Payment totals are the caller's
/// job (FIFO applies to many documents under one payment lock).
fn apply_to_document<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment: &ArPaymentModel,
    overlay: &CustomerInvoiceModel,
    amount: Decimal,
    now: NaiveDateTime,
) -> Result<ArPaymentApplicationModel, ArError> {
    let existing = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::PaymentId.eq(payment.id))
        .filter(ApplicationColumn::InvoiceId.eq(overlay.id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let stored = if let Some(row) = existing {
        let mut changes = ArPaymentApplicationRecord::new();
        changes
            .set_id(row.id)
            .set_applied_amount(row.applied_amount + amount)
            .set_applied_at(now)
            .set_updated_at(now);
        changes
            .update(executor)
            .map_err(|error| ArError::Database(error.to_string()))?;
        ApplicationEntity::find()
            .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(ApplicationColumn::Id.eq(row.id))
            .find_one(executor)
            .map_err(|error| ArError::Database(error.to_string()))?
            .ok_or_else(|| ArError::Database("application vanished after update".to_string()))?
    } else {
        let mut record = ArPaymentApplicationRecord::new();
        record
            .set_id(Uuid::new_v4())
            .set_tenant_id(session.tenant_id.clone())
            .set_legal_entity_id(payment.legal_entity_id)
            .set_payment_id(payment.id)
            .set_invoice_id(overlay.id)
            .set_applied_amount(amount)
            .set_applied_at(now)
            .set_created_at(now)
            .set_updated_at(now)
            .set_applied_by(Some(session.subject_id));
        record
            .insert(executor)
            .map_err(|error| ArError::Database(error.to_string()))?;
        ApplicationEntity::find()
            .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(ApplicationColumn::PaymentId.eq(payment.id))
            .filter(ApplicationColumn::InvoiceId.eq(overlay.id))
            .find_one(executor)
            .map_err(|error| ArError::Database(error.to_string()))?
            .ok_or_else(|| ArError::Database("application vanished after insert".to_string()))?
    };
    let mut event = ArPaymentApplicationEventRecord::new();
    event
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(payment.legal_entity_id)
        .set_application_id(stored.id)
        .set_kind("APPLIED".to_string())
        .set_amount(amount)
        .set_actor_id(Some(session.subject_id))
        .set_occurred_at(now);
    event
        .insert(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    Ok(stored)
}

fn persist_payment_totals<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment: &ArPaymentModel,
    added: Decimal,
    now: NaiveDateTime,
) -> Result<ArPaymentModel, ArError> {
    let mut changes = ArPaymentRecord::new();
    changes
        .set_id(payment.id)
        .set_applied_amount(payment.applied_amount + added)
        .set_unapplied_amount(payment.unapplied_amount - added)
        .set_updated_at(now)
        .set_updated_by(Some(session.subject_id));
    changes
        .update(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::Id.eq(payment.id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or_else(|| ArError::Database("payment vanished after totals".to_string()))
}
fn require_applicable(payment: &ArPaymentModel) -> Result<&ArPaymentModel, ArError> {
    if payment.status != "POSTED" {
        return Err(ArError::Conflict(
            "payment is not in an applicable state".to_string(),
        ));
    }
    Ok(payment)
}

fn heal_overlay_row<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    document: &AccountingPostedDocumentModel,
) -> Result<CustomerInvoiceModel, ArError> {
    refresh_overlay_row(executor, session, document.legal_entity_id, document)?;
    OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::InvoiceId.eq(document.id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or_else(|| ArError::Database("overlay row missing after refresh".to_string()))
}

pub fn create_payment_application<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment_id: &str,
    customer_invoice_id: &str,
    applied_amount: &str,
) -> Result<Value, ArError> {
    let document_id = parse_uuid(customer_invoice_id, "customer_invoice_id")?;
    let amount = parse_positive_amount(applied_amount, "applied_amount")?;
    let payment = find_payment(executor, session, payment_id)?;
    payment_lock(executor, payment.id)?;
    // Reload the receipt under the lock: a concurrent allocation on the
    // same receipt commits first, so the unapplied guard and the totals
    // write act on committed truth (no lost update on applied/unapplied).
    let payment = find_payment(executor, session, payment_id)?;
    require_applicable(&payment)?;
    if amount > payment.unapplied_amount {
        return Err(ArError::Conflict(
            "application exceeds the payment's unapplied balance".to_string(),
        ));
    }
    let document = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::Id.eq(document_id))
        .filter(DocumentColumn::DocumentType.is_in(["CUSTOMER_INVOICE", "CUSTOMER_DEBIT_NOTE"]))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| {
            row.legal_entity_id == payment.legal_entity_id && row.customer_id == payment.customer_id
        })
        .ok_or(ArError::NotFound)?;
    document_lock(executor, document.id)?;
    if document.currency_code != payment.currency_code {
        return Err(ArError::Conflict(
            "payment and document currencies differ".to_string(),
        ));
    }
    let overlay = heal_overlay_row(executor, session, &document)?;
    let capacity = document_capacity(executor, session, &document)?;
    if amount > capacity {
        return Err(ArError::Conflict(
            "application exceeds the document's outstanding balance".to_string(),
        ));
    }
    let now = Utc::now().naive_utc();
    let stored = apply_to_document(executor, session, &payment, &overlay, amount, now)?;
    let updated = persist_payment_totals(executor, session, &payment, amount, now)?;
    refresh_overlay_row(executor, session, updated.legal_entity_id, &document)?;
    Ok(application_json(
        &stored,
        document.id,
        &payment.currency_code,
    ))
}

pub fn list_payment_applications<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment_id: &str,
) -> Result<Value, ArError> {
    let payment = find_payment(executor, session, payment_id)?;
    let rows = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::PaymentId.eq(payment.id))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    // Rows FK the overlay; the contract speaks open-item (document) ids.
    let overlay_ids: Vec<Uuid> = rows
        .iter()
        .filter(|row| row.applied_amount > Decimal::ZERO)
        .map(|row| row.invoice_id)
        .collect();
    let overlay_rows = OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::Id.is_in(overlay_ids))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let document_of: std::collections::HashMap<Uuid, Uuid> = overlay_rows
        .into_iter()
        .map(|row| (row.id, row.invoice_id))
        .collect();
    let mut items: Vec<(NaiveDateTime, Value)> = rows
        .into_iter()
        .filter(|row| row.applied_amount > Decimal::ZERO)
        .filter_map(|row| {
            let document_id = *document_of.get(&row.invoice_id)?;
            Some((
                row.applied_at,
                application_json(&row, document_id, &payment.currency_code),
            ))
        })
        .collect();
    items.sort_by_key(|(applied_at, item)| (*applied_at, item["customer_invoice_id"].to_string()));
    Ok(json!(items
        .into_iter()
        .map(|(_, item)| item)
        .collect::<Vec<_>>()))
}

/// Contract `auto_apply`: FIFO over the customer's open items, oldest
/// `due_date` first, until the payment's unapplied balance is exhausted.
pub fn auto_apply_fifo<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment_id: &str,
) -> Result<Value, ArError> {
    let payment = find_payment(executor, session, payment_id)?;
    payment_lock(executor, payment.id)?;
    // Reload under the lock before spending unapplied balance (same rule
    // as the manual allocation path).
    let payment = find_payment(executor, session, payment_id)?;
    require_applicable(&payment)?;
    let mut remaining = payment.unapplied_amount;
    if remaining <= Decimal::ZERO {
        return Ok(payment_json(&payment));
    }
    let mut documents = ledger::chargeback_documents(
        executor,
        session,
        &[payment.legal_entity_id],
        Some(&payment.currency_code),
    )?
    .into_iter()
    .filter(|row| row.customer_id == payment.customer_id)
    .collect::<Vec<_>>();
    documents.sort_by(|left, right| {
        left.due_date
            .cmp(&right.due_date)
            .then_with(|| left.document_number.cmp(&right.document_number))
    });
    let now = Utc::now().naive_utc();
    let mut allocated = Decimal::ZERO;
    for document in &documents {
        if remaining <= Decimal::ZERO {
            break;
        }
        // Same lock order as the manual path: payment first, then document.
        document_lock(executor, document.id)?;
        let overlay = heal_overlay_row(executor, session, document)?;
        let capacity = document_capacity(executor, session, document)?;
        if capacity <= Decimal::ZERO {
            continue;
        }
        let take = capacity.min(remaining);
        apply_to_document(executor, session, &payment, &overlay, take, now)?;
        refresh_overlay_row(executor, session, payment.legal_entity_id, document)?;
        remaining -= take;
        allocated += take;
    }
    let updated = if allocated > Decimal::ZERO {
        persist_payment_totals(executor, session, &payment, allocated, now)?
    } else {
        payment
    };
    // Stamp derived from stored facts (like the replay path reads them),
    // never from the in-memory clock: a fresh creation and its idempotent
    // replay must answer value-for-value.
    let stamps = crate::payments::last_applied_stamps(executor, session, &[updated.id])?;
    Ok(crate::payments::payment_json_stamped(
        &updated,
        stamps.get(&updated.id).copied(),
    ))
}

/// Contract reversal: the pair row's applied amount returns IN FULL to the
/// payment's unapplied balance and the document's outstanding. The pair row
/// itself survives at zero (migration 0009) because the event journal FKs
/// it — history is immutable, state is in-place. Reversal is journaled as
/// a REVERSED event (actor, time, reason) and is itself replayable under
/// the GL idempotency discipline.
pub fn reverse_payment_application<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    application_id: &str,
    idempotency_key: &str,
    reason: &str,
) -> Result<Value, ArError> {
    let id = parse_uuid(application_id, "id")?;
    let scope = ledger::resolve_scope_entities(executor, session, None)?;
    // Pre-lock read exists only to resolve scope and locate the payment
    // to lock; every MUTATED snapshot is reloaded after payment_lock.
    let row = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| scope.contains(&row.legal_entity_id))
        .ok_or(ArError::NotFound)?;
    let fingerprint = crate::payments::reverse_fingerprint(
        session.tenant_id.as_str(),
        row.legal_entity_id,
        id,
        reason,
    );
    payment_lock(executor, row.payment_id)?;
    // Reload under the lock: a concurrent reversal of THIS row (or of a
    // sibling row on the same payment) commits before we are allowed to
    // proceed, so the guard and the totals below act on committed truth.
    let row = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| scope.contains(&row.legal_entity_id))
        .ok_or(ArError::NotFound)?;
    let payment = PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::Id.eq(row.payment_id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or(ArError::NotFound)?;
    // Idempotency lookup INSIDE the lock: a matching retry that queued on
    // payment_lock now sees the first request's committed record and
    // re-serves it, instead of acting on stale snapshots and surfacing a
    // duplicate-key 409 (or a bogus "already reversed").
    if let Some(record) =
        crate::payments::find_idempotency(executor, session, row.legal_entity_id, idempotency_key)?
    {
        return crate::payments::replay_payment(executor, session, &record, &fingerprint);
    }
    let overlay = OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::Id.eq(row.invoice_id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or(ArError::NotFound)?;
    let document = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::Id.eq(overlay.invoice_id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or(ArError::NotFound)?;
    document_lock(executor, document.id)?;
    require_applicable(&payment)?;
    if row.applied_amount <= Decimal::ZERO {
        return Err(ArError::Conflict(
            "application is already reversed".to_string(),
        ));
    }
    let now = Utc::now().naive_utc();
    let mut changes = ArPaymentApplicationRecord::new();
    changes
        .set_id(row.id)
        .set_applied_amount(Decimal::ZERO)
        .set_updated_at(now);
    changes
        .update(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let mut event = ArPaymentApplicationEventRecord::new();
    event
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(row.legal_entity_id)
        .set_application_id(row.id)
        .set_kind("REVERSED".to_string())
        .set_amount(row.applied_amount)
        .set_actor_id(Some(session.subject_id))
        .set_occurred_at(now)
        .set_reason(Some(reason.to_string()));
    event
        .insert(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let updated = persist_payment_totals(executor, session, &payment, -row.applied_amount, now)?;
    refresh_overlay_row(executor, session, row.legal_entity_id, &document)?;
    crate::payments::record_completed(
        executor,
        session,
        row.legal_entity_id,
        idempotency_key,
        &fingerprint,
        "application_reversal",
        payment.id,
        now,
    )?;
    let stamps = crate::payments::last_applied_stamps(executor, session, &[updated.id])?;
    Ok(crate::payments::payment_json_stamped(
        &updated,
        stamps.get(&updated.id).copied(),
    ))
}

#[cfg(test)]
#[allow(
    clippy::too_many_lines,
    reason = "live end-to-end scenarios read top to bottom"
)]
mod tests {
    use super::*;

    #[test]
    fn amount_guard_rejects_non_positive_and_garbage() {
        assert!(parse_positive_amount("0", "applied_amount").is_err());
        assert!(parse_positive_amount("-5", "applied_amount").is_err());
        assert!(parse_positive_amount("abc", "applied_amount").is_err());
        assert_eq!(
            parse_positive_amount("12.5", "applied_amount").expect("valid"),
            Decimal::new(125, 1)
        );
    }

    #[test]
    fn lock_namespaces_are_distinct_from_siblings() {
        let target = Uuid::new_v4();
        let key = |namespace: &[u8]| {
            const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
            const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
            let mut hash = FNV_OFFSET;
            for byte in namespace.iter().chain(target.as_bytes().iter()) {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            i64::from_be_bytes(hash.to_be_bytes())
        };
        let payment = key(b"ar-payment-application");
        let document = key(b"ar-application-document");
        let overlay = key(b"ar-overlay-projection");
        let sequence = key(b"ar-payment-sequence");
        assert_ne!(payment, document);
        assert_ne!(payment, overlay);
        assert_ne!(payment, sequence);
        assert_ne!(document, overlay);
        assert_ne!(document, sequence);
        assert_ne!(overlay, sequence);
    }

    #[test]
    fn application_json_echoes_the_open_item_document_id() {
        // Built via a stored row? Keep this test dependency-free by
        // asserting the shape produced from a synthetic row read-back is
        // covered live; here just the money normalization helper.
        assert_eq!(money(Decimal::new(30_500_000, 6)), "30.5");
        assert_eq!(money(Decimal::ZERO), "0");
    }

    // -------------------------------------------------------------------
    // Live acceptance (disposable DB, migrations 0001..0009, psql on PATH).
    // Helpers duplicate the payments suite shape so this file is runnable
    // standalone, same as the ledger/projection suites.
    // -------------------------------------------------------------------

    fn run_hex() -> String {
        static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        TAG.get_or_init(|| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX));
            let mix = (now ^ (u64::from(std::process::id()) << 32))
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                >> 24
                & 0x00FF_FFFF;
            format!("{mix:06x}")
        })
        .clone()
    }

    fn uid(slot: &str, run: &str) -> String {
        format!("{slot}{run}0-0000-4000-8000-000000000000")
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
            std::env::var("DB_PASS").unwrap_or_else(|_| String::new()),
        )
        .env(
            "PGDATABASE",
            std::env::var("DB_NAME").unwrap_or_else(|_| "rerp".into()),
        )
        .args(["-v", "ON_ERROR_STOP=1", "-q", "-At"]);
    }

    /// Seeds via STDIN (`CodeQL`: never write constructed SQL to a temp file).
    fn psql_run(sql: &str) -> (bool, String, String) {
        use std::io::Write;
        use std::process::Stdio;
        let mut cmd = std::process::Command::new("psql");
        psql_env(&mut cmd);
        cmd.stdin(Stdio::piped());
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

    fn psql_session_query(session_sql: &str, body_sql: &str) -> String {
        let sql = format!("BEGIN;\n{session_sql}\n{body_sql};\nCOMMIT;");
        let (ok, stdout, stderr) = psql_run(&sql);
        assert!(ok, "query failed: {stderr}");
        stdout.trim().to_string()
    }

    fn session_guc(tenant: &str, subject: &str, org: &str, app: &str) -> String {
        format!(
            "SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
                '{app}', '[\"billing\"]'::jsonb,
                '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');"
        )
    }

    fn session_for(tenant: String, subject: Uuid, org: Uuid) -> SessionContext {
        SessionContext {
            tenant_id: tenant,
            subject_id: subject,
            organization_id: org,
            session_id: "ar-applications-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:ledger:read".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        }
    }

    fn pay(
        session: &SessionContext,
        amount: &str,
        customer: &str,
        entity: &str,
    ) -> Result<Value, ArError> {
        let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = format!(
            "app-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        crate::http_support::with_ar_transaction(session, |executor| {
            crate::payments::create_payment(
                executor,
                session,
                &key,
                amount,
                "usd",
                "cash",
                &today,
                customer,
                Some(entity),
                None,
                Some("applications-acceptance"),
                false,
            )
        })
    }

    fn apply(
        session: &SessionContext,
        payment_id: &str,
        document_id: &str,
        amount: &str,
    ) -> Result<Value, ArError> {
        crate::http_support::with_ar_transaction(session, |executor| {
            create_payment_application(executor, session, payment_id, document_id, amount)
        })
    }

    fn apps(session: &SessionContext, payment_id: &str) -> Result<Value, ArError> {
        crate::http_support::with_ar_transaction(session, |executor| {
            list_payment_applications(executor, session, payment_id)
        })
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0009) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_applications_allocate_guard_journal_and_fifo() {
        use chrono::TimeDelta;
        // One tenant, one entity: doc A (100, due future) and doc B (60,
        // overdue) belong to `cust`; doc C (40) to `cust2`. A foreign
        // tenant holds doc X for the same customer uuid. NO overlay rows
        // are seeded — the allocation flow must heal them itself.
        let run = run_hex();
        let tenant = format!("ar-app-{run}");
        let other = format!("ar-app-x-{run}");
        let org = Uuid::new_v4();
        let xorg = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity = uid("1", &run);
        let period = uid("8", &run);
        let doc_a = uid("a", &run);
        let doc_b = uid("b", &run);
        let doc_c = uid("c", &run);
        let doc_x = uid("d", &run);
        let credit_c = uid("e", &run);
        let cust = Uuid::new_v4().to_string();
        let cust2 = Uuid::new_v4().to_string();
        let guc = session_guc(&tenant, &subject.to_string(), &org.to_string(), "ar-app-a");
        let guc_x = session_guc(&other, &subject.to_string(), &xorg.to_string(), "ar-app-x");
        let today = Utc::now().date_naive();
        let day = |offset: i64| {
            (today + TimeDelta::days(offset))
                .format("%Y-%m-%d")
                .to_string()
        };

        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org}', 'Applications Entity', 'USD', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{period}', '{tenant}', '{entity}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by, original_document_id)
VALUES
    ('{doc_a}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-A',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'a',
     '2026-02-01', '{day_at_20}', 'USD', 2, 100, 0, 0, 100.000000, now(),
     '{subject}'::uuid, NULL),
    ('{doc_b}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-B',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'b',
     '2026-02-02', '{due_b}', 'USD', 2, 60, 0, 0, 60.000000, now(),
     '{subject}'::uuid, NULL),
    ('{doc_c}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-C',
     'CUSTOMER_INVOICE', '{cust2}', 'app-test', 'fixture', 'c',
     '2026-02-03', '{day_at_10}', 'USD', 2, 40, 0, 0, 40.000000, now(),
     '{subject}'::uuid, NULL),
    ('{credit_c}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-1',
     'CUSTOMER_CREDIT_NOTE', '{cust2}', 'app-test', 'fixture', 'credit-c',
     '2026-02-05', '{day_at_10}', 'USD', 2, 10, 0, 0, 10.000000, now(),
     '{subject}'::uuid, '{doc_c}');
COMMIT;",
            day_at_20 = day(20),
            due_b = day(-15),
            day_at_10 = day(10),
        ));
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_x}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{uid_x_entity}', '{other}', '{xorg}', 'Applications Foreign', 'USD', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{xperiod}', '{other}', '{uid_x_entity}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by, original_document_id)
VALUES
    ('{doc_x}', '{other}', '{uid_x_entity}', '{xperiod}', '{other}-INV-X',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'x',
     '2026-02-04', '{day_future}', 'USD', 2, 50, 0, 0, 50.000000, now(),
     '{subject}'::uuid, NULL);
COMMIT;",
            uid_x_entity = uid("2", &run),
            xperiod = uid("9", &run),
            day_future = day(30),
        ));

        let session = session_for(tenant.clone(), subject, org);
        let foreign = session_for(other.clone(), subject, xorg);

        // Receipt 1: 80 for cust. Fresh receipts are internally POSTED and
        // contract-PENDING; allocation is legal regardless of GL posting.
        let p1 = pay(&session, "80", &cust, &entity).expect("p1");
        assert_eq!(p1["status"], "PENDING");

        // Allocate 30 then 10 to doc A: pair accumulates into ONE row.
        let a1 = apply(&session, p1["id"].as_str().unwrap(), &doc_a, "30").expect("apply 30");
        assert_eq!(a1["customer_invoice_id"], doc_a, "open-item id space");
        assert_eq!(a1["applied_amount"], "30");
        assert_eq!(a1["currency_code"], "USD");
        assert!(a1["id"].is_string());
        let a2 = apply(&session, p1["id"].as_str().unwrap(), &doc_a, "10").expect("apply 10");
        assert_eq!(a2["id"], a1["id"], "same pair row, accumulated");
        assert_eq!(a2["applied_amount"], "40");
        let list1 = apps(&session, p1["id"].as_str().unwrap()).expect("list p1");
        assert_eq!(list1.as_array().map(Vec::len), Some(1));
        assert_eq!(list1[0]["applied_amount"], "40");

        // Every delta is journaled as its own event row.
        let journal = psql_session_query(
            &guc,
            &format!(
                "SELECT count(*), sum(e.amount) FROM ar_payment_application_events e
                   JOIN ar_payment_applications ap ON ap.id = e.application_id
                  WHERE ap.payment_id = '{}'",
                p1["id"].as_str().unwrap()
            ),
        );
        assert_eq!(journal, "2|40.000000");
        // The journal is append-only at the SCHEMA level, not by convention.
        let (mutated, _, _) = psql_run(&format!(
            "BEGIN;\n{guc}\nUPDATE ar_payment_application_events SET amount = amount;\nROLLBACK;"
        ));
        assert!(!mutated, "runtime role must not edit journal rows");

        // Payment totals + contract status follow the allocations. The
        // contract Payment carries no allocation columns — probe storage
        // directly; the JSON carries the derived status + applied_at stamp.
        let totals_after = psql_session_query(
            &guc,
            &format!(
                "SELECT applied_amount, unapplied_amount FROM ar_payments WHERE id = '{}'",
                p1["id"].as_str().unwrap()
            ),
        );
        assert_eq!(totals_after, "40.000000|40.000000");
        let p1_after = crate::http_support::with_ar_transaction(&session, |executor| {
            crate::payments::get_payment(executor, &session, p1["id"].as_str().unwrap())
        })
        .expect("get p1");
        assert_eq!(p1_after["status"], "APPLIED_PARTIAL");
        assert!(
            p1_after["applied_at"].is_string(),
            "last allocation instant is exposed"
        );

        // Guards: over-unapplied, over-capacity, bad input, foreign target.
        assert!(
            matches!(
                apply(&session, p1["id"].as_str().unwrap(), &doc_a, "100"),
                Err(ArError::Conflict(_))
            ),
            "exceeds payment unapplied"
        );
        assert!(matches!(
            apply(&session, p1["id"].as_str().unwrap(), &doc_a, "0"),
            Err(ArError::Validation(_))
        ));
        assert!(matches!(
            apply(&session, p1["id"].as_str().unwrap(), &doc_a, "-4"),
            Err(ArError::Validation(_))
        ));
        assert!(matches!(
            apply(&session, p1["id"].as_str().unwrap(), "not-a-uuid", "5"),
            Err(ArError::Validation(_))
        ));
        assert!(
            matches!(
                apply(&session, p1["id"].as_str().unwrap(), &doc_x, "5"),
                Err(ArError::NotFound)
            ),
            "foreign tenant document is invisible"
        );
        // A payment cannot allocate to ANOTHER customer's document, and the
        // denial does not leak existence.
        assert!(matches!(
            apply(&session, p1["id"].as_str().unwrap(), &doc_c, "5"),
            Err(ArError::NotFound)
        ));
        // Credit notes net invoices; they are not allocation targets (same
        // chargeback-type rule as the open-item read axis).
        assert!(
            matches!(
                apply(&session, p1["id"].as_str().unwrap(), &credit_c, "5"),
                Err(ArError::NotFound)
            ),
            "credit notes are not chargeback documents"
        );

        // Capacity guard on its own axis: cust2's receipt has ample
        // unapplied but doc C nets to 40 − 10 (credit) = 30.
        let p2 = pay(&session, "200", &cust2, &entity).expect("p2");
        assert!(
            matches!(
                apply(&session, p2["id"].as_str().unwrap(), &doc_c, "50"),
                Err(ArError::Conflict(_))
            ),
            "exceeds document capacity"
        );
        apply(&session, p2["id"].as_str().unwrap(), &doc_c, "30").expect("doc c full");
        // Doc C is now paid — a further allocation is a capacity conflict.
        assert!(matches!(
            apply(&session, p2["id"].as_str().unwrap(), &doc_c, "1"),
            Err(ArError::Conflict(_))
        ));

        // Drain p1 on the overdue doc B: status becomes APPLIED.
        apply(&session, p1["id"].as_str().unwrap(), &doc_b, "40").expect("p1 drained");
        let p1_done = crate::http_support::with_ar_transaction(&session, |executor| {
            crate::payments::get_payment(executor, &session, p1["id"].as_str().unwrap())
        })
        .expect("get p1 done");
        assert_eq!(p1_done["status"], "APPLIED");
        let totals_done = psql_session_query(
            &guc,
            &format!(
                "SELECT applied_amount, unapplied_amount FROM ar_payments WHERE id = '{}'",
                p1["id"].as_str().unwrap()
            ),
        );
        assert_eq!(totals_done, "80.000000|0.000000");
        // An exhausted payment can no longer allocate anything.
        assert!(matches!(
            apply(&session, p1["id"].as_str().unwrap(), &doc_a, "1"),
            Err(ArError::Conflict(_))
        ));

        // The overlay cache reflects the allocations through the projection:
        // A: 100 − 40 = 60, B: 60 − 40 = 20 (both rows healed on first apply).
        let overlay_a = psql_session_query(
            &guc,
            &format!(
                "SELECT outstanding_amount FROM customer_invoices WHERE invoice_id = '{doc_a}'"
            ),
        );
        let overlay_b = psql_session_query(
            &guc,
            &format!(
                "SELECT outstanding_amount FROM customer_invoices WHERE invoice_id = '{doc_b}'"
            ),
        );
        assert_eq!(overlay_a, "60.000000");
        assert_eq!(overlay_b, "20.000000");

        // FIFO auto-apply: 50 more from cust. Overdue doc B first (cap 20),
        // then doc A (cap 60) takes the remaining 30 in p3's own pair row.
        let p3 = pay(&session, "50", &cust, &entity).expect("p3");
        let auto = crate::http_support::with_ar_transaction(&session, |executor| {
            auto_apply_fifo(executor, &session, p3["id"].as_str().unwrap())
        })
        .expect("fifo");
        assert_eq!(auto["status"], "APPLIED");
        assert!(auto["applied_at"].is_string());
        let list3 = apps(&session, p3["id"].as_str().unwrap()).expect("list p3");
        assert_eq!(list3.as_array().map(Vec::len), Some(2));
        // Both FIFO rows share one applied_at, so match by document — never
        // by list position.
        let by_doc: std::collections::HashMap<&str, &str> = list3
            .as_array()
            .expect("array")
            .iter()
            .map(|row| {
                (
                    row["customer_invoice_id"].as_str().unwrap_or_default(),
                    row["applied_amount"].as_str().unwrap_or_default(),
                )
            })
            .collect();
        assert_eq!(by_doc.get(doc_b.as_str()), Some(&"20"));
        assert_eq!(by_doc.get(doc_a.as_str()), Some(&"30"));
        // One pair row per (payment, document): p1's row carries its
        // accumulated 30+10 = 40; p3's is a separate 30.
        let pair_a = psql_session_query(
            &guc,
            &format!(
                "SELECT count(*), sum(applied_amount) FROM ar_payment_applications
                  WHERE invoice_id = (SELECT id FROM customer_invoices WHERE invoice_id = '{doc_a}')"
            ),
        );
        assert_eq!(pair_a, "2|70.000000", "p1 row (40) + p3 row (30) for A");
        let a_stored = psql_session_query(
            &guc,
            &format!(
                "SELECT max(applied_amount) FROM ar_payment_applications
                  WHERE invoice_id = (SELECT id FROM customer_invoices WHERE invoice_id = '{doc_a}')"
            ),
        );
        assert_eq!(a_stored, "40.000000", "p1 row holds the accumulated 30+10");

        // Reversal (F2c): take p3's 30 back off doc A.
        let app_a3 = list3
            .as_array()
            .expect("array")
            .iter()
            .find(|row| row["customer_invoice_id"].as_str() == Some(doc_a.as_str()))
            .and_then(|row| row["id"].as_str())
            .expect("p3 row for A")
            .to_string();
        let rev_key = |n: u64| format!("rev-{run}-{n}");
        let reason = "customer returned goods";
        let rev1 = crate::http_support::with_ar_transaction(&session, |executor| {
            reverse_payment_application(executor, &session, &app_a3, &rev_key(1), reason)
        })
        .expect("reverse p3/A");
        assert_eq!(rev1["id"], p3["id"], "reversal answers with the payment");
        assert_eq!(rev1["status"], "APPLIED_PARTIAL");
        let list3b = apps(&session, p3["id"].as_str().unwrap()).expect("list p3 after reverse");
        assert_eq!(list3b.as_array().map(Vec::len), Some(1));
        assert_eq!(list3b[0]["customer_invoice_id"], doc_b, "only B survives");
        let p3_totals = psql_session_query(
            &guc,
            &format!(
                "SELECT applied_amount, unapplied_amount FROM ar_payments WHERE id = '{}'",
                p3["id"].as_str().unwrap()
            ),
        );
        assert_eq!(
            p3_totals, "20.000000|30.000000",
            "money returned to the receipt"
        );
        let journal_a3 = psql_session_query(
            &guc,
            &format!(
                "SELECT string_agg(kind || ':' || amount::text, ',' ORDER BY occurred_at, id)
                   FROM ar_payment_application_events WHERE application_id = '{app_a3}'"
            ),
        );
        assert_eq!(journal_a3, "APPLIED:30.000000,REVERSED:30.000000");
        let overlay_a_after = psql_session_query(
            &guc,
            &format!(
                "SELECT outstanding_amount FROM customer_invoices WHERE invoice_id = '{doc_a}'"
            ),
        );
        assert_eq!(overlay_a_after, "60.000000", "100 - only p1's 40 remains");

        // Guards: double reverse conflicts; replay of the identical body
        // re-serves without touching balances; key reuse with a different
        // reason is a fingerprint conflict; foreign sessions get 404.
        assert!(
            matches!(
                crate::http_support::with_ar_transaction(&session, |executor| {
                    reverse_payment_application(executor, &session, &app_a3, &rev_key(2), reason)
                }),
                Err(ArError::Conflict(_))
            ),
            "already reversed"
        );
        let replay = crate::http_support::with_ar_transaction(&session, |executor| {
            reverse_payment_application(executor, &session, &app_a3, &rev_key(1), reason)
        })
        .expect("replay");
        assert_eq!(replay["status"], "APPLIED_PARTIAL");
        let p3_after_replay = psql_session_query(
            &guc,
            &format!(
                "SELECT applied_amount, unapplied_amount FROM ar_payments WHERE id = '{}'",
                p3["id"].as_str().unwrap()
            ),
        );
        assert_eq!(p3_after_replay, p3_totals, "replay moved nothing");
        assert!(
            matches!(
                crate::http_support::with_ar_transaction(&session, |executor| {
                    reverse_payment_application(
                        executor,
                        &session,
                        &app_a3,
                        &rev_key(1),
                        "a different story entirely",
                    )
                }),
                Err(ArError::Conflict(_))
            ),
            "reason is part of the fingerprint"
        );
        assert!(matches!(
            crate::http_support::with_ar_transaction(&foreign, |executor| {
                reverse_payment_application(executor, &foreign, &app_a3, &rev_key(3), reason)
            }),
            Err(ArError::NotFound)
        ));

        // Re-apply accumulates onto the SAME (reversed) pair row and is
        // journaled as a fresh APPLIED event; history stays intact.
        let reappled = apply(&session, p3["id"].as_str().unwrap(), &doc_a, "15").expect("re-apply");
        assert_eq!(
            reappled["id"], app_a3,
            "pair row identity survives reversal"
        );
        assert_eq!(reappled["applied_amount"], "15");
        let journal_a3b = psql_session_query(
            &guc,
            &format!(
                "SELECT string_agg(kind || ':' || amount::text, ',' ORDER BY occurred_at, id)
                   FROM ar_payment_application_events WHERE application_id = '{app_a3}'"
            ),
        );
        assert_eq!(
            journal_a3b,
            "APPLIED:30.000000,REVERSED:30.000000,APPLIED:15.000000"
        );
        let overlay_a_final = psql_session_query(
            &guc,
            &format!(
                "SELECT outstanding_amount FROM customer_invoices WHERE invoice_id = '{doc_a}'"
            ),
        );
        assert_eq!(overlay_a_final, "45.000000", "100 - 40 - 15");

        // Nothing left for a fresh cust2 receipt: FIFO is a no-op, not an error.
        let p4 = pay(&session, "10", &cust2, &entity).expect("p4");
        let noop = crate::http_support::with_ar_transaction(&session, |executor| {
            auto_apply_fifo(executor, &session, p4["id"].as_str().unwrap())
        })
        .expect("fifo noop");
        assert_eq!(noop["status"], "PENDING");
        assert!(noop["applied_at"].is_null(), "never applied, never stamped");

        // Foreign sessions see neither the payments nor their allocations.
        for probe in [
            apps(&foreign, p1["id"].as_str().unwrap()),
            apply(&foreign, p1["id"].as_str().unwrap(), &doc_a, "1"),
        ] {
            assert!(matches!(probe, Err(ArError::NotFound)), "peek: {probe:?}");
        }
        let foreign_events =
            psql_session_query(&guc_x, "SELECT count(*) FROM ar_payment_application_events");
        assert_eq!(foreign_events, "0", "journal is tenant-RLS bounded");
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0009) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_f2d_receipt_identity_acceptance() {
        // Epic F2 closer: the identity invoice + credit + receipt +
        // allocation + outstanding must hold value-for-value across the
        // READ path (composed open items), the CACHE (overlay rows and the
        // reconciliation projection), and the immutable EVENTS journal —
        // through full, partial, and multi-document receipts, over-balance
        // and over-payment rejections, a reversal, cross-tenant probes,
        // and idempotent replays.
        // Distinct run tag so uuid slots never collide with the F2b
        // scenario sharing this module's process-wide run_hex(): keep the
        // first five digits and force the sixth to a value that differs
        // from run_hex()'s own, making identity collision impossible.
        let hex = run_hex();
        let run = format!(
            "{}{}",
            &hex[..5],
            if hex.ends_with('f') { 'e' } else { 'f' }
        );
        let tenant = format!("ar-f2d-{run}");
        let other = format!("ar-f2d-x-{run}");
        let org = Uuid::new_v4();
        let xorg = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity = uid("1", &run);
        let period = uid("8", &run);
        let doc_a = uid("a", &run); // 500 gross, -100 credit => 400 net
        let doc_b = uid("b", &run); // 300
        let doc_c = uid("c", &run); // 60
        let credit_a = uid("e", &run); // 100 against doc A
        let entity_x = uid("2", &run);
        let period_x = uid("9", &run);
        let doc_x = uid("d", &run); // foreign twin for the same customer
        let cust = Uuid::new_v4().to_string();
        let guc = session_guc(&tenant, &subject.to_string(), &org.to_string(), "ar-f2d-a");
        let guc_x = session_guc(&other, &subject.to_string(), &xorg.to_string(), "ar-f2d-x");
        let today = Utc::now().date_naive();
        let day = |offset: i64| {
            (today + chrono::TimeDelta::days(offset))
                .format("%Y-%m-%d")
                .to_string()
        };

        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org}', 'F2D Entity', 'USD', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{period}', '{tenant}', '{entity}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by, original_document_id)
VALUES
    ('{doc_a}', '{tenant}', '{entity}', '{period}', '{tenant}-F2DA',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'a',
     '2026-02-01', '{day30}', 'USD', 2, 500, 0, 0, 500.000000, now(),
     '{subject}'::uuid, NULL),
    ('{doc_b}', '{tenant}', '{entity}', '{period}', '{tenant}-F2DB',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'b',
     '2026-02-02', '{day10}', 'USD', 2, 300, 0, 0, 300.000000, now(),
     '{subject}'::uuid, NULL),
    ('{doc_c}', '{tenant}', '{entity}', '{period}', '{tenant}-F2DC',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'c',
     '2026-02-03', '{day45}', 'USD', 2, 60, 0, 0, 60.000000, now(),
     '{subject}'::uuid, NULL),
    ('{credit_a}', '{tenant}', '{entity}', '{period}', '{tenant}-F2DX',
     'CUSTOMER_CREDIT_NOTE', '{cust}', 'app-test', 'fixture', 'credit-a',
     '2026-02-05', '{day20}', 'USD', 2, 100, 0, 0, 100.000000, now(),
     '{subject}'::uuid, '{doc_a}');
COMMIT;",
            day30 = day(30),
            day10 = day(10),
            day45 = day(45),
            day20 = day(20),
        ));
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_x}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity_x}', '{other}', '{xorg}', 'F2D Foreign', 'USD', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{period_x}', '{other}', '{entity_x}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by, original_document_id)
VALUES
    ('{doc_x}', '{other}', '{entity_x}', '{period_x}', '{other}-F2DX',
     'CUSTOMER_INVOICE', '{cust}', 'app-test', 'fixture', 'x',
     '2026-02-01', '{day30}', 'USD', 2, 500, 0, 0, 500.000000, now(),
     '{subject}'::uuid, NULL);
COMMIT;",
            day30 = day(30),
        ));

        let session = session_for(tenant.clone(), subject, org);
        let foreign = session_for(other.clone(), subject, xorg);
        let today_str = today.format("%Y-%m-%d").to_string();
        let pay_key = |key: &str, amount: &str| {
            crate::http_support::with_ar_transaction(&session, |executor| {
                crate::payments::create_payment(
                    executor,
                    &session,
                    key,
                    amount,
                    "usd",
                    "cash",
                    &today_str,
                    &cust,
                    Some(&entity),
                    None,
                    Some("f2d-acceptance"),
                    false,
                )
            })
        };
        let items = |session: &SessionContext, filters: &ledger::ListFilters| {
            crate::http_support::with_ar_transaction(session, |executor| {
                ledger::list_open_items(executor, session, filters)
            })
        };
        let item = |id: &str| {
            crate::http_support::with_ar_transaction(&session, |executor| {
                ledger::get_open_item(executor, &session, id)
            })
        };
        let default_filters = || ledger::ListFilters {
            customer_id: Some(Uuid::parse_str(&cust).expect("cust")),
            ..ledger::ListFilters::default()
        };

        // Read path BEFORE receipts: A nets 400 (credit already settles it
        // -> PARTIAL), B and C sit CONFIRMED at gross.
        let before = items(&session, &default_filters()).expect("open items");
        assert_eq!(before["total"], 3);
        let by_id = |payload: &Value, id: &str| {
            payload["items"]
                .as_array()
                .expect("items")
                .iter()
                .find(|row| row["id"].as_str() == Some(id))
                .cloned()
                .expect("row")
        };
        let a_before = by_id(&before, &doc_a);
        assert_eq!(a_before["original_amount"], "400", "credit-netted original");
        assert_eq!(a_before["outstanding_amount"], "400");
        assert_eq!(a_before["status"], "PARTIAL", "credits already settled 100");
        assert_eq!(by_id(&before, &doc_b)["status"], "CONFIRMED");

        // Full receipt R1: 400 -> A goes PAID and leaves the default view.
        let r1 = pay_key("f2d-r1", "400").expect("r1");
        apply(&session, r1["id"].as_str().unwrap(), &doc_a, "400").expect("r1 to A");
        let a_paid = item(&doc_a).expect("item A");
        assert_eq!(a_paid["status"], "PAID");
        assert_eq!(a_paid["outstanding_amount"], "0");
        assert_eq!(a_paid["payment_history"].as_array().map(Vec::len), Some(1));

        // Partial receipt R2: 100 -> B PARTIAL at 200.
        let r2 = pay_key("f2d-r2", "100").expect("r2");
        apply(&session, r2["id"].as_str().unwrap(), &doc_b, "100").expect("r2 to B");
        let b_partial = item(&doc_b).expect("item B");
        assert_eq!(b_partial["status"], "PARTIAL");
        assert_eq!(b_partial["outstanding_amount"], "200");

        // Multi-document receipt R3: 200 -> B (closing it) and 60 -> C.
        let r3 = pay_key("f2d-r3", "500").expect("r3");
        apply(&session, r3["id"].as_str().unwrap(), &doc_b, "200").expect("r3 to B");
        apply(&session, r3["id"].as_str().unwrap(), &doc_c, "60").expect("r3 to C");
        let after_r3 = psql_session_query(
            &guc,
            &format!(
                "SELECT applied_amount, unapplied_amount FROM ar_payments WHERE id = '{}'",
                r3["id"].as_str().unwrap()
            ),
        );
        assert_eq!(
            after_r3, "260.000000|240.000000",
            "one receipt, two documents"
        );
        let after_r3_list = items(&session, &default_filters()).expect("after r3");
        assert_eq!(after_r3_list["total"], 0, "default view is UNPAID balances");
        let paid_filter = ledger::ListFilters {
            status: Some("PAID".to_string()),
            ..default_filters()
        };
        let paid_list = items(&session, &paid_filter).expect("paid list");
        assert_eq!(paid_list["total"], 3, "A, B, C all settled");
        let b_history = item(&doc_b).expect("item B");
        assert_eq!(
            b_history["payment_history"].as_array().map(Vec::len),
            Some(2)
        );

        // Rejections: an over-BALANCE application (capacity 0 on C) and an
        // over-PAYMENT application (amount beyond R4's unapplied).
        assert!(
            matches!(
                apply(&session, r3["id"].as_str().unwrap(), &doc_c, "240"),
                Err(ArError::Conflict(message)) if message.contains("outstanding")
            ),
            "over-balance is 409"
        );
        let r4 = pay_key("f2d-r4", "100").expect("r4");
        assert!(
            matches!(
                apply(&session, r4["id"].as_str().unwrap(), &doc_a, "101"),
                Err(ArError::Conflict(message)) if message.contains("unapplied")
            ),
            "over-payment is 409"
        );

        // Identity across the three axes, value-for-value:
        //   original (invoice - credits) == applied + outstanding (cache),
        //   journal net (APPLIED - REVERSED) == applied (rows),
        //   per receipt: applied + unapplied == amount.
        let identity = psql_session_query(
            &guc,
            &format!(
                "SELECT string_agg(right(d.document_number, 4)
                           || ':' || (d.total_amount - COALESCE(cr.credit, 0))::text
                           || ':' || o.original_amount::text
                           || ':' || (COALESCE(ap.app, 0) + o.outstanding_amount)::text
                           || ':' || o.outstanding_amount::text
                           || ':' || COALESCE(j.net, 0)::text, ',' ORDER BY d.document_number)
                   FROM customer_invoices o
                   JOIN accounting_posted_documents d ON d.id = o.invoice_id
                   LEFT JOIN (
                       SELECT original_document_id AS doc, sum(total_amount) AS credit
                         FROM accounting_posted_documents
                        WHERE document_type = 'CUSTOMER_CREDIT_NOTE'
                        GROUP BY 1) cr ON cr.doc = d.id
                   LEFT JOIN (
                       SELECT invoice_id, sum(applied_amount) AS app
                         FROM ar_payment_applications GROUP BY 1) ap ON ap.invoice_id = o.id
                   LEFT JOIN (
                       SELECT a.invoice_id,
                              sum(CASE e.kind WHEN 'APPLIED' THEN e.amount ELSE -e.amount END) AS net
                         FROM ar_payment_applications a
                         JOIN ar_payment_application_events e ON e.application_id = a.id
                        GROUP BY 1) j ON j.invoice_id = o.id
                  WHERE d.customer_id = '{cust}'
                    AND d.document_type = 'CUSTOMER_INVOICE'"
            ),
        );
        assert_eq!(
            identity,
            "F2DA:400.000000:400.000000:400.000000:0.000000:400.000000,\
             F2DB:300.000000:300.000000:300.000000:0.000000:300.000000,\
             F2DC:60.000000:60.000000:60.000000:0.000000:60.000000",
            "invoice - credit == applied + outstanding; \
             journal net (APPLIED - REVERSED) == applied"
        );
        assert_eq!(
            psql_session_query(
                &guc,
                "SELECT count(*) FROM ar_payments WHERE applied_amount + unapplied_amount <> payment_amount"
            ),
            "0",
            "every receipt balances applied + unapplied"
        );
        assert_eq!(
            psql_session_query(
                &guc,
                "SELECT count(*) FROM (
                     SELECT a.id, sum(CASE e.kind WHEN 'APPLIED' THEN e.amount ELSE -e.amount END) AS net
                       FROM ar_payment_applications a
                       JOIN ar_payment_application_events e ON e.application_id = a.id
                      GROUP BY a.id
                    HAVING sum(CASE e.kind WHEN 'APPLIED' THEN e.amount ELSE -e.amount END)
                           IS DISTINCT FROM max(a.applied_amount)) t"
            ),
            "0",
            "journal net equals live allocation rows per pair"
        );
        let report = crate::http_support::with_ar_transaction(&session, |executor| {
            crate::projection::project_settlement_state(
                executor,
                &session,
                Uuid::parse_str(&entity).expect("entity"),
            )
        })
        .expect("projection");
        assert_eq!(report.documents, 3);
        assert_eq!(report.refreshed, 0, "cache already agreed with derivation");
        assert_eq!(
            report.ar_control_drift, None,
            "no GL control account seeded here -> honestly not computable \
             (the F1c projection suite pins zero-drift with journal lines)"
        );

        // Reversal of R2's allocation returns the 100 to the receipt and
        // re-opens B; the audit spine keeps both sides of the story.
        let r2_rows = apps(&session, r2["id"].as_str().unwrap()).expect("r2 rows");
        let app_b = r2_rows[0]["id"].as_str().unwrap().to_string();
        let reason = "duplicate bank deposit";
        let reversed = crate::http_support::with_ar_transaction(&session, |executor| {
            reverse_payment_application(executor, &session, &app_b, "f2d-rev-1", reason)
        })
        .expect("reverse");
        assert_eq!(reversed["status"], "PENDING", "R2 fully un-applied again");
        let b_open = item(&doc_b).expect("item B after reverse");
        assert_eq!(b_open["status"], "PARTIAL");
        assert_eq!(b_open["outstanding_amount"], "100");
        assert_eq!(
            b_open["payment_history"].as_array().map(Vec::len),
            Some(1),
            "reversed allocation leaves the history view"
        );
        let replay_rev = crate::http_support::with_ar_transaction(&session, |executor| {
            reverse_payment_application(executor, &session, &app_b, "f2d-rev-1", reason)
        })
        .expect("replay reversal");
        assert_eq!(replay_rev, reversed, "reversal replay is value-for-value");
        assert!(
            matches!(
                crate::http_support::with_ar_transaction(&session, |executor| {
                    reverse_payment_application(executor, &session, &app_b, "f2d-rev-2", reason)
                }),
                Err(ArError::Conflict(_))
            ),
            "double reverse is 409"
        );

        // Identity survives the reversal on all three axes: B row net 200,
        // outstanding 100; journal net = 100 + 200 - 100 = 200.
        let identity_after = psql_session_query(
            &guc,
            &format!(
                "SELECT right(d.document_number, 4) || ':' || o.original_amount::text
                           || ':' || o.outstanding_amount::text || ':' || j.net::text
                   FROM customer_invoices o
                   JOIN accounting_posted_documents d ON d.id = o.invoice_id
                   JOIN (
                       SELECT a.invoice_id,
                              sum(CASE e.kind WHEN 'APPLIED' THEN e.amount ELSE -e.amount END) AS net
                         FROM ar_payment_applications a
                         JOIN ar_payment_application_events e ON e.application_id = a.id
                        GROUP BY 1) j ON j.invoice_id = o.id
                  WHERE d.document_number = '{tenant}-F2DB'"
            ),
        );
        assert_eq!(identity_after, "F2DB:300.000000:100.000000:200.000000");
        let report2 = crate::http_support::with_ar_transaction(&session, |executor| {
            crate::projection::project_settlement_state(
                executor,
                &session,
                Uuid::parse_str(&entity).expect("entity"),
            )
        })
        .expect("projection 2");
        assert_eq!(report2.refreshed, 0, "reversal kept the cache honest");
        let open_after = items(&session, &default_filters()).expect("after reverse");
        assert_eq!(open_after["total"], 1, "only B reopened");
        assert_eq!(by_id(&open_after, &doc_b)["outstanding_amount"], "100");

        // Idempotent receipt replay: stable value-for-value and reflecting
        // the CURRENT state of the receipt (R1 has been allocated since its
        // creation, and the replay honestly re-serves that state).
        let r1_replay = pay_key("f2d-r1", "400").expect("replay r1");
        assert_eq!(r1_replay["id"], r1["id"]);
        assert_eq!(
            r1_replay["status"], "APPLIED",
            "replay serves current state"
        );
        assert!(!r1_replay["applied_at"].is_null());
        let r1_replay2 = pay_key("f2d-r1", "400").expect("replay r1 again");
        assert_eq!(r1_replay2, r1_replay, "replays are stable");
        assert_eq!(
            psql_session_query(&guc, "SELECT count(*) FROM ar_payments"),
            "4",
            "three receipts + R4, replays stored nothing"
        );

        // Cross-tenant: the foreign org with the SAME customer uuid sees
        // only its own document, and cannot even probe ours.
        let foreign_items = items(&foreign, &default_filters()).expect("foreign items");
        assert_eq!(foreign_items["total"], 1, "only the foreign twin");
        assert_eq!(foreign_items["items"][0]["id"], doc_x);
        assert!(item(&doc_a).is_ok(), "sanity: ours visible to us");
        let foreign_probe = crate::http_support::with_ar_transaction(&foreign, |executor| {
            ledger::get_open_item(executor, &foreign, &doc_a)
        });
        assert!(matches!(foreign_probe, Err(ArError::NotFound)));
        assert!(
            matches!(
                apply(&foreign, r1["id"].as_str().unwrap(), &doc_b, "1"),
                Err(ArError::NotFound)
            ),
            "no allocation into another tenant"
        );
    }
}
