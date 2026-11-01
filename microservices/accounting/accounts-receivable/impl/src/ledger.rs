//! Derived customer open items over the immutable posted facts.
//!
//! The ledger is a READ model: outstanding balances come from
//! `accounting_posted_documents` (invoices and customer debit notes,
//! net of linked credit notes) plus the rebuildable AR overlay's
//! settlement state (write-offs, payment allocations from F2 onward).
//! Aging buckets and days-overdue are computed at read time so they can
//! never drift from `due_date`. The OpenItem id IS the posted document
//! id — stable, derived, and identical in `list` and `get`.

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rerp_entities::accounting::foundation::legal_entity::{
    AccountingLegalEntityModel, Column as LegalEntityColumn, Entity as LegalEntityEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    AccountingPostedDocumentModel, Column as DocumentColumn, Entity as DocumentEntity,
};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use uuid::Uuid;

use rerp_accounting_accounts_receivable::models::ar_payment_application::{
    ArPaymentApplicationModel, Column as ApplicationColumn, Entity as ApplicationEntity,
};
use rerp_accounting_accounts_receivable::models::customer_invoice::{
    Column as OverlayColumn, CustomerInvoiceModel, Entity as OverlayEntity,
};

const DEFAULT_LIMIT: i32 = 20;
const MAX_LIMIT: i32 = 100;
const CHARGEBACK_TYPES: [&str; 2] = ["CUSTOMER_INVOICE", "CUSTOMER_DEBIT_NOTE"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArError {
    Validation(String),
    Policy(String),
    NotFound,
    Database(String),
}

impl std::fmt::Display for ArError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Policy(message) => write!(formatter, "accounting scope rejected: {message}"),
            Self::NotFound => write!(formatter, "resource not found"),
            Self::Database(message) => write!(formatter, "database failure: {message}"),
        }
    }
}

/// One open item with everything the derivation needs, decoupled from
/// persistence so the arithmetic is unit-testable.
pub struct ItemFacts<'a> {
    pub document: &'a AccountingPostedDocumentModel,
    pub credit_notes: Decimal,
    pub overlay: Option<&'a CustomerInvoiceModel>,
    pub applied: Decimal,
    pub today: NaiveDate,
}

fn money(value: Decimal) -> String {
    value.round_dp(6).normalize().to_string()
}

/// Aging bucket from the due date; `INVALID` is unreachable because the
/// schema guarantees a real date — the enum member exists for legacy rows.
#[must_use]
pub fn aging_bucket(due_date: NaiveDate, today: NaiveDate) -> &'static str {
    let overdue = days_overdue(due_date, today);
    match overdue {
        0 => "CURRENT",
        1..=30 => "1-30",
        31..=60 => "31-60",
        61..=90 => "61-90",
        _ => "90+",
    }
}

#[must_use]
pub fn days_overdue(due_date: NaiveDate, today: NaiveDate) -> i64 {
    (today - due_date).num_days().max(0)
}

/// Terms are derived from the posting-time gap; anything else is CUSTOM
/// (the overlay can refine it in F2 when payment-terms master data lands).
#[must_use]
pub fn derive_terms(document_date: NaiveDate, due_date: NaiveDate) -> &'static str {
    let gap = (due_date - document_date).num_days();
    match gap {
        0 => "DUE_ON_RECEIPT",
        15 => "NET_15",
        30 => "NET_30",
        45 => "NET_45",
        60 => "NET_60",
        _ => "CUSTOM",
    }
}

fn derive_status(
    overlay: Option<&CustomerInvoiceModel>,
    outstanding: Decimal,
    applied: Decimal,
) -> String {
    if let Some(state) = overlay.map(|row| row.status.as_str()) {
        if matches!(state, "WRITTEN_OFF" | "DISPUTED") {
            return state.to_string();
        }
    }
    if outstanding <= Decimal::ZERO {
        return "PAID".to_string();
    }
    if applied > Decimal::ZERO {
        return "PARTIAL".to_string();
    }
    "CONFIRMED".to_string()
}

/// Pure derivation: no executor, no I/O — the whole point of F1b.
#[must_use]
pub fn compose_item(facts: &ItemFacts<'_>) -> Value {
    let document = facts.document;
    let write_off = facts.overlay.map_or(Decimal::ZERO, |row| {
        row.write_off_amount.unwrap_or(Decimal::ZERO)
    });
    let raw_outstanding = document.total_amount - facts.credit_notes - facts.applied - write_off;
    let outstanding = raw_outstanding.max(Decimal::ZERO);
    let updated_at = facts
        .overlay
        .map_or(document.created_at, |row| row.updated_at);
    json!({
        "id": document.id.to_string(),
        "invoice_id": document.id.to_string(),
        "customer_id": document.customer_id.to_string(),
        "original_amount": money(document.total_amount - facts.credit_notes),
        "outstanding_amount": money(outstanding),
        "currency_code": document.currency_code,
        "status": derive_status(facts.overlay, raw_outstanding, facts.applied),
        "aging_bucket": aging_bucket(document.due_date, facts.today),
        "company_id": document.legal_entity_id.to_string(),
        "due_date": document.due_date.to_string(),
        "terms": derive_terms(document.document_date, document.due_date),
        "credit_limit_check": Value::Null,
        "created_at": rfc3339(document.created_at),
        "updated_at": rfc3339(updated_at),
    })
}

fn rfc3339(value: NaiveDateTime) -> String {
    value.and_utc().to_rfc3339()
}

pub struct ListFilters {
    pub status: Option<String>,
    pub aging_bucket: Option<String>,
    pub customer_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub currency_code: Option<String>,
    pub days_overdue_min: Option<i64>,
    pub days_overdue_max: Option<i64>,
    pub search: Option<String>,
    pub page: i32,
    pub limit: i32,
}

/// Pagination defaults mirror the contract (page 1, limit 20) — never 0.
impl Default for ListFilters {
    fn default() -> Self {
        Self {
            status: None,
            aging_bucket: None,
            customer_id: None,
            company_id: None,
            currency_code: None,
            days_overdue_min: None,
            days_overdue_max: None,
            search: None,
            page: 1,
            limit: DEFAULT_LIMIT,
        }
    }
}

// One flat argument per contract query parameter (codegen shape).
#[allow(clippy::too_many_arguments)]
pub fn parse_list_filters(
    status: Option<&str>,
    aging_bucket: Option<&str>,
    customer_id: Option<&str>,
    company_id: Option<&str>,
    currency_code: Option<&str>,
    days_overdue_min: Option<i32>,
    days_overdue_max: Option<i32>,
    search: Option<&str>,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<ListFilters, ArError> {
    let uuid = |value: &str, field: &str| -> Result<Uuid, ArError> {
        Uuid::parse_str(value).map_err(|_| ArError::Validation(format!("{field} must be a UUID")))
    };
    Ok(ListFilters {
        status: status.map(str::to_uppercase),
        aging_bucket: aging_bucket.map(str::to_string),
        customer_id: customer_id.map(|v| uuid(v, "customer_id")).transpose()?,
        company_id: company_id.map(|v| uuid(v, "company_id")).transpose()?,
        currency_code: currency_code.map(|v| v.to_uppercase()),
        days_overdue_min: days_overdue_min.map(i64::from),
        days_overdue_max: days_overdue_max.map(i64::from),
        search: search
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(str::to_string),
        page: page.unwrap_or(1).max(1),
        limit: {
            let value = limit.unwrap_or(DEFAULT_LIMIT);
            if !(1..=MAX_LIMIT).contains(&value) {
                return Err(ArError::Validation(format!(
                    "limit must be between 1 and {MAX_LIMIT}"
                )));
            }
            value
        },
    })
}

fn resolve_legal_entity<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    company_id: Option<Uuid>,
) -> Result<AccountingLegalEntityModel, ArError> {
    let mut query = LegalEntityEntity::find()
        .filter(LegalEntityColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(LegalEntityColumn::IsActive.eq(true));
    if let Some(company) = company_id {
        query = query.filter(LegalEntityColumn::Id.eq(company));
    } else {
        query = query.filter(LegalEntityColumn::OrganizationId.eq(session.organization_id));
    }
    match query.find_one(executor) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(ArError::Policy(
            "no active legal entity matches this scope".to_string(),
        )),
        Err(error) => Err(ArError::Database(error.to_string())),
    }
}

fn chargeback_documents<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
    currency_code: Option<&str>,
) -> Result<Vec<AccountingPostedDocumentModel>, ArError> {
    let mut query = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.is_in(CHARGEBACK_TYPES));
    if let Some(currency) = currency_code {
        query = query.filter(DocumentColumn::CurrencyCode.eq(currency.to_string()));
    }
    query
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))
}

fn credit_note_totals<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
) -> Result<HashMap<Uuid, Decimal>, ArError> {
    let notes = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.eq("CUSTOMER_CREDIT_NOTE".to_string()))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let mut totals: HashMap<Uuid, Decimal> = HashMap::new();
    for note in notes {
        if let Some(original) = note.original_document_id {
            *totals.entry(original).or_insert(Decimal::ZERO) += note.total_amount;
        }
    }
    Ok(totals)
}

fn overlay_state<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity: Uuid,
    document_ids: Vec<Uuid>,
) -> Result<HashMap<Uuid, (CustomerInvoiceModel, Decimal)>, ArError> {
    if document_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let overlays = OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::LegalEntityId.eq(legal_entity))
        .filter(OverlayColumn::InvoiceId.is_in(document_ids))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let overlay_ids: Vec<Uuid> = overlays.iter().map(|row| row.id).collect();
    let applications: Vec<ArPaymentApplicationModel> = if overlay_ids.is_empty() {
        Vec::new()
    } else {
        ApplicationEntity::find()
            .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(ApplicationColumn::LegalEntityId.eq(legal_entity))
            .filter(ApplicationColumn::InvoiceId.is_in(overlay_ids))
            .all(executor)
            .map_err(|error| ArError::Database(error.to_string()))?
    };
    let mut applied: HashMap<Uuid, Decimal> = HashMap::new();
    for application in applications {
        *applied
            .entry(application.invoice_id)
            .or_insert(Decimal::ZERO) += application.applied_amount;
    }
    Ok(overlays
        .into_iter()
        .map(|row| {
            let sum = applied.get(&row.id).copied().unwrap_or(Decimal::ZERO);
            (row.invoice_id, (row, sum))
        })
        .collect())
}

fn matches_filters(item: &Value, document_number: &str, filters: &ListFilters) -> bool {
    if let Some(status) = &filters.status {
        if item["status"] != json!(status) {
            return false;
        }
    }
    if let Some(bucket) = &filters.aging_bucket {
        if item["aging_bucket"] != json!(bucket) {
            return false;
        }
    }
    if let Some(customer) = filters.customer_id {
        if item["customer_id"] != json!(customer.to_string()) {
            return false;
        }
    }
    if let Some(min) = filters.days_overdue_min {
        if days_from_item(item) < min {
            return false;
        }
    }
    if let Some(max) = filters.days_overdue_max {
        if days_from_item(item) > max {
            return false;
        }
    }
    if let Some(needle) = &filters.search {
        let haystack = format!(
            "{document_number} {}",
            item["status"].as_str().unwrap_or_default()
        )
        .to_lowercase();
        if !haystack.contains(&needle.to_lowercase()) {
            return false;
        }
    }
    true
}

fn days_from_item(item: &Value) -> i64 {
    NaiveDate::parse_from_str(item["due_date"].as_str().unwrap_or_default(), "%Y-%m-%d")
        .map(|due| days_overdue(due, Utc::now().date_naive()))
        .unwrap_or(0)
}

pub fn list_open_items<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    filters: &ListFilters,
) -> Result<Value, ArError> {
    let legal_entity = resolve_legal_entity(executor, session, filters.company_id)?;
    let documents = chargeback_documents(
        executor,
        session,
        legal_entity.id,
        filters.currency_code.as_deref(),
    )?;
    let document_ids: Vec<Uuid> = documents.iter().map(|row| row.id).collect();
    let credits = credit_note_totals(executor, session, legal_entity.id)?;
    let overlays = overlay_state(executor, session, legal_entity.id, document_ids)?;
    let today = Utc::now().date_naive();

    let mut items: Vec<(NaiveDate, String, Value)> = Vec::with_capacity(documents.len());
    for document in &documents {
        let facts = ItemFacts {
            document,
            credit_notes: credits.get(&document.id).copied().unwrap_or(Decimal::ZERO),
            overlay: overlays.get(&document.id).map(|(row, _)| row),
            applied: overlays
                .get(&document.id)
                .map_or(Decimal::ZERO, |(_, sum)| *sum),
            today,
        };
        let item = compose_item(&facts);
        if matches_filters(&item, &document.document_number, filters) {
            items.push((document.due_date, document.document_number.clone(), item));
        }
    }
    items.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let total = items.len();
    let offset = usize::try_from((filters.page - 1) * filters.limit).unwrap_or(usize::MAX);
    let page_items: Vec<Value> = items
        .into_iter()
        .skip(offset)
        .take(usize::try_from(filters.limit).unwrap_or(usize::MAX))
        .map(|(_, _, item)| item)
        .collect();
    Ok(json!({
        "items": page_items,
        "total": total,
        "page": filters.page,
        "limit": filters.limit,
        "has_more": offset + page_items.len() < total,
    }))
}

pub fn get_open_item<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    id: &str,
) -> Result<Value, ArError> {
    let id =
        Uuid::parse_str(id).map_err(|_| ArError::Validation("id must be a UUID".to_string()))?;
    let document = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| CHARGEBACK_TYPES.contains(&row.document_type.as_str()))
        .ok_or(ArError::NotFound)?;
    let credits = credit_note_totals(executor, session, document.legal_entity_id)?;
    let overlays = overlay_state(executor, session, document.legal_entity_id, vec![id])?;
    let today = Utc::now().date_naive();
    let facts = ItemFacts {
        document: &document,
        credit_notes: credits.get(&id).copied().unwrap_or(Decimal::ZERO),
        overlay: overlays.get(&id).map(|(row, _)| row),
        applied: overlays.get(&id).map_or(Decimal::ZERO, |(_, sum)| *sum),
        today,
    };
    let mut item = compose_item(&facts);
    let history: Vec<Value> = overlays
        .get(&id)
        .map(|(overlay, _)| application_rows(executor, session, overlay.id))
        .transpose()?
        .unwrap_or_default();
    item["days_overdue"] = json!(days_overdue(document.due_date, today));
    item["payment_history"] = json!(history);
    Ok(item)
}

fn application_rows<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    overlay_id: Uuid,
) -> Result<Vec<Value>, ArError> {
    ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::InvoiceId.eq(overlay_id))
        .all(executor)
        .map(|rows| {
            rows.into_iter()
                .map(|row| {
                    json!({
                        "id": row.id.to_string(),
                        "payment_id": row.payment_id.to_string(),
                        "customer_invoice_id": row.invoice_id.to_string(),
                        "applied_amount": money(row.applied_amount),
                    })
                })
                .collect()
        })
        .map_err(|error| ArError::Database(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").expect("test date")
    }

    #[test]
    fn aging_bucket_boundaries() {
        let today = date("2026-07-31");
        assert_eq!(aging_bucket(date("2026-07-31"), today), "CURRENT");
        assert_eq!(aging_bucket(date("2026-07-30"), today), "1-30");
        assert_eq!(aging_bucket(date("2026-07-01"), today), "1-30");
        assert_eq!(aging_bucket(date("2026-06-30"), today), "31-60");
        assert_eq!(aging_bucket(date("2026-05-02"), today), "61-90");
        assert_eq!(aging_bucket(date("2026-05-01"), today), "90+");
    }

    #[test]
    fn days_overdue_never_negative() {
        let today = date("2026-07-31");
        assert_eq!(days_overdue(date("2026-09-01"), today), 0);
        assert_eq!(days_overdue(date("2026-07-21"), today), 10);
    }

    #[test]
    fn terms_map_to_the_canonical_gaps() {
        let start = date("2026-01-01");
        assert_eq!(derive_terms(start, start), "DUE_ON_RECEIPT");
        assert_eq!(derive_terms(start, date("2026-01-16")), "NET_15");
        assert_eq!(derive_terms(start, date("2026-01-31")), "NET_30");
        assert_eq!(derive_terms(start, date("2026-02-15")), "NET_45");
        assert_eq!(derive_terms(start, date("2026-03-02")), "NET_60");
        assert_eq!(derive_terms(start, date("2026-02-20")), "CUSTOM");
    }

    #[test]
    fn limit_validation_rejects_out_of_range() {
        assert!(parse_list_filters(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(0)
        )
        .is_err());
        assert!(parse_list_filters(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(101)
        )
        .is_err());
        assert!(parse_list_filters(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(100)
        )
        .is_ok());
    }

    fn run_hex() -> String {
        static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        TAG.get_or_init(|| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            let mix = (now ^ ((std::process::id() as u64) << 32))
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                >> 24
                & 0x00FF_FFFF;
            format!("{mix:06x}")
        })
        .clone()
    }

    fn uid(slot: &str, run: &str) -> String {
        format!("d{slot}{run}-0000-4000-8000-000000000000")
    }

    fn psql_seed(sql: &str) {
        use std::process::Command;
        let mut path = std::env::temp_dir();
        path.push(format!("rerp-ar-ledger-{}.sql", std::process::id()));
        std::fs::write(&path, sql).expect("write sql");
        let output = Command::new("psql")
            .env(
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
            .args(["-v", "ON_ERROR_STOP=1", "-q", "-At", "-f"])
            .arg(&path)
            .output()
            .expect("psql on PATH");
        let _ = std::fs::remove_file(&path);
        assert!(
            output.status.success(),
            "seed failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0006) and psql on PATH"]
    fn live_open_items_derive_from_posted_facts() {
        // Two invoices for one tenant: one untouched (CONFIRMED, CURRENT),
        // one carrying a credit note, an overlay, a payment application and
        // a 15-day overdue due date (PARTIAL, "1-30"). A second tenant with
        // an untouched invoice must be invisible from the first session.
        let run = run_hex();
        let tenant = format!("ar-ledger-{run}");
        let other = format!("ar-ledger-x-{run}");
        let org = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity = uid("1", &run);
        let period = uid("8", &run);
        let clean = uid("d", &run);
        let mixed = uid("e", &run);
        let credit = uid("c", &run);
        let overlay = uid("f", &run);
        let payment = uid("a", &run);
        let application = uid("b", &run);
        let other_doc = uid("9", &run);
        let xperiod = uid("4", &run);
        let today = Utc::now().date_naive();
        let overdue = (today - TimeDelta::days(15)).format("%Y-%m-%d").to_string();
        let future = (today + TimeDelta::days(20)).format("%Y-%m-%d").to_string();

        psql_seed(&format!(
            "BEGIN;
SET search_path TO rerp, public;
SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
    'ar-ledger-test', '[\"billing\"]'::jsonb,
    '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org}', 'Ledger Test', 'GBP', true, now(), now());
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
    ('{clean}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-1',
     'CUSTOMER_INVOICE', '{payment}', 'ledger-test', 'fixture', 'one',
     '2026-01-05', '{future}', 'GBP', 2, 100, 0, 20, 120.000000, now(),
     '{subject}'::uuid, NULL),
    ('{mixed}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-2',
     'CUSTOMER_INVOICE', '{payment}', 'ledger-test', 'fixture', 'two',
     '2026-01-06', '{overdue}', 'GBP', 2, 50, 0, 10, 60.000000, now(),
     '{subject}'::uuid, NULL),
    ('{credit}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-1',
     'CUSTOMER_CREDIT_NOTE', '{payment}', 'ledger-test', 'fixture', 'credit',
     '2026-01-07', '{overdue}', 'GBP', 2, 16, 0, 4, 20.000000, now(),
     '{subject}'::uuid, '{mixed}');
INSERT INTO customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, updated_at)
VALUES ('{overlay}', '{tenant}', '{entity}', '{mixed}', '{payment}',
     60.000000, 30.000000, 'GBP', '{overdue}', 'NET_30', now());
INSERT INTO ar_payments
    (id, tenant_id, legal_entity_id, customer_id, payment_number, payment_date,
     payment_amount, currency_code, payment_method, status, updated_at)
VALUES ('{payment}', '{tenant}', '{entity}', '{payment}', '{tenant}-PMT-1',
     '2026-01-08', 10.000000, 'GBP', 'WIRE', 'DRAFT', now());
INSERT INTO ar_payment_applications
    (id, tenant_id, legal_entity_id, payment_id, invoice_id, applied_amount)
VALUES ('{application}', '{tenant}', '{entity}', '{payment}', '{overlay}', 10.000000);
COMMIT;
BEGIN;
SELECT public.rls_set_session('{other}',
    '{subject}'::uuid, '{org}'::uuid, 'ar-ledger-other',
    '[\"billing\"]'::jsonb, '[\"accounting:invoice:write\"]'::jsonb,
    'service', 'shipper');
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{period}', '{other}', '{org}', 'Ledger Other', 'GBP', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{xperiod}', '{other}', '{period}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by)
VALUES ('{other_doc}', '{other}', '{period}', '{xperiod}', '{other}-INV-1',
     'CUSTOMER_INVOICE', '{payment}', 'ledger-test', 'fixture', 'other',
     '2026-01-08', '{future}', 'GBP', 2, 40, 0, 8, 48.000000, now(), '{subject}'::uuid);
COMMIT;"
        ));

        let session = SessionContext {
            tenant_id: tenant.clone(),
            subject_id: subject,
            organization_id: org,
            session_id: "ar-ledger-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:ledger:read".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        };
        let filters =
            parse_list_filters(None, None, None, None, None, None, None, None, None, None)
                .expect("default filters");
        let listed = crate::http_support::with_ar_transaction(&session, |executor| {
            list_open_items(executor, &session, &filters)
        })
        .expect("list open items");
        assert_eq!(
            listed["total"], 2,
            "credit notes are not open items themselves"
        );
        let clean_row = listed["items"]
            .as_array()
            .expect("items array")
            .iter()
            .find(|row| row["invoice_id"] == json!(clean.to_string()))
            .expect("clean invoice");
        assert_eq!(clean_row["status"], "CONFIRMED");
        assert_eq!(clean_row["outstanding_amount"], "120");
        assert_eq!(clean_row["aging_bucket"], "CURRENT");
        assert_eq!(clean_row["credit_limit_check"], Value::Null);
        let mixed_row = listed["items"]
            .as_array()
            .expect("items array")
            .iter()
            .find(|row| row["invoice_id"] == json!(mixed.to_string()))
            .expect("mixed invoice");
        assert_eq!(mixed_row["status"], "PARTIAL");
        assert_eq!(mixed_row["outstanding_amount"], "30");
        assert_eq!(
            mixed_row["original_amount"], "40",
            "credit note nets the original"
        );
        assert_eq!(mixed_row["aging_bucket"], "1-30");

        let partial_only = ListFilters {
            status: Some("PARTIAL".to_string()),
            ..Default::default()
        };
        let filtered = crate::http_support::with_ar_transaction(&session, |executor| {
            list_open_items(executor, &session, &partial_only)
        })
        .expect("filtered list");
        assert_eq!(filtered["total"], 1);
        assert_eq!(filtered["items"][0]["invoice_id"], json!(mixed.to_string()));

        let detail = crate::http_support::with_ar_transaction(&session, |executor| {
            get_open_item(executor, &session, &mixed.to_string())
        })
        .expect("get open item");
        assert_eq!(detail["days_overdue"], 15);
        assert_eq!(
            detail["payment_history"].as_array().expect("history").len(),
            1
        );
        assert_eq!(detail["payment_history"][0]["applied_amount"], "10");

        // Other tenant's document is invisible under RLS -> NotFound.
        let cross = crate::http_support::with_ar_transaction(&session, |executor| {
            get_open_item(executor, &session, &other_doc.to_string())
        });
        assert!(matches!(cross, Err(ArError::NotFound)));
    }
}
