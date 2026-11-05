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
    Column as LegalEntityColumn, Entity as LegalEntityEntity,
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
use rerp_accounting_accounts_receivable::models::ar_payment_application_event::{
    Column as EventColumn, Entity as EventEntity,
};
use rerp_accounting_accounts_receivable::models::customer_invoice::{
    Column as OverlayColumn, CustomerInvoiceModel, Entity as OverlayEntity,
};

const DEFAULT_LIMIT: i32 = 20;
const MAX_LIMIT: i32 = 100;
const CHARGEBACK_TYPES: [&str; 2] = ["CUSTOMER_INVOICE", "CUSTOMER_DEBIT_NOTE"];
const MAX_SOURCE_DOCUMENTS: i64 = 50_000;
const MAX_CREDIT_NOTES: i64 = 50_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArError {
    Validation(String),
    Policy(String),
    NotFound,
    Conflict(String),
    Database(String),
}

impl std::fmt::Display for ArError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Policy(message) => write!(formatter, "accounting scope rejected: {message}"),
            Self::NotFound => write!(formatter, "resource not found"),
            Self::Conflict(message) => write!(formatter, "conflict: {message}"),
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
    settled: Decimal,
) -> String {
    if let Some(state) = overlay.map(|row| row.status.as_str()) {
        if matches!(state, "WRITTEN_OFF" | "DISPUTED") {
            return state.to_string();
        }
    }
    if outstanding <= Decimal::ZERO {
        return "PAID".to_string();
    }
    if settled > Decimal::ZERO {
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
        "status": derive_status(
            facts.overlay,
            raw_outstanding,
            facts.applied + facts.credit_notes,
        ),
        "aging_bucket": aging_bucket(document.due_date, facts.today),
        "company_id": document.legal_entity_id.to_string(),
        "due_date": document.due_date.to_string(),
        "terms": derive_terms(document.document_date, document.due_date),
        "credit_limit_check": Value::Null,
        "created_at": rfc3339(document.created_at),
        "updated_at": rfc3339(updated_at),
    })
}

/// Net outstanding before clamping (negative = over-collected).
#[must_use]
pub fn raw_outstanding(facts: &ItemFacts<'_>) -> Decimal {
    let write_off = facts.overlay.map_or(Decimal::ZERO, |row| {
        row.write_off_amount.unwrap_or(Decimal::ZERO)
    });
    facts.document.total_amount - facts.credit_notes - facts.applied - write_off
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

/// The caller's active legal entities within their organization. An
/// explicit company_id narrows inside the org, never escapes it; an
/// unscoped read covers ALL org entities — never an arbitrary one when
/// the org has several.
pub(crate) fn resolve_scope_entities<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    company_id: Option<Uuid>,
) -> Result<Vec<Uuid>, ArError> {
    let mut query = LegalEntityEntity::find()
        .filter(LegalEntityColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(LegalEntityColumn::OrganizationId.eq(session.organization_id))
        .filter(LegalEntityColumn::IsActive.eq(true));
    if let Some(company) = company_id {
        query = query.filter(LegalEntityColumn::Id.eq(company));
    }
    let entities = query
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if entities.is_empty() {
        return Err(ArError::Policy(
            "no active legal entity matches this scope".to_string(),
        ));
    }
    let mut ids: Vec<Uuid> = entities.into_iter().map(|row| row.id).collect();
    ids.sort();
    Ok(ids)
}

pub(crate) fn chargeback_documents<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    currency_code: Option<&str>,
) -> Result<Vec<AccountingPostedDocumentModel>, ArError> {
    let mut query = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.is_in(CHARGEBACK_TYPES));
    if let Some(currency) = currency_code {
        query = query.filter(DocumentColumn::CurrencyCode.eq(currency.to_string()));
    }
    // One bounded statement: read MAX+1 rows and reject over the limit, so
    // READ COMMITTED races cannot smuggle extra rows past the guard
    // (SQL paging arrives with the F1c projection).
    let documents = query
        .limit((MAX_SOURCE_DOCUMENTS + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if documents.len() as i64 > MAX_SOURCE_DOCUMENTS {
        return Err(ArError::Validation(format!(
            "open-item source exceeds the {MAX_SOURCE_DOCUMENTS}-document read bound; narrow with currency_code"
        )));
    }
    Ok(documents)
}

pub(crate) fn credit_note_totals<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    document_ids: Vec<Uuid>,
) -> Result<HashMap<Uuid, Decimal>, ArError> {
    if document_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let notes = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.eq("CUSTOMER_CREDIT_NOTE".to_string()))
        .filter(DocumentColumn::OriginalDocumentId.is_in(document_ids))
        .limit((MAX_CREDIT_NOTES + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if notes.len() as i64 > MAX_CREDIT_NOTES {
        return Err(ArError::Validation(format!(
            "linked credit notes exceed the {MAX_CREDIT_NOTES}-row read bound"
        )));
    }
    let mut totals: HashMap<Uuid, Decimal> = HashMap::new();
    for note in notes {
        if let Some(original) = note.original_document_id {
            *totals.entry(original).or_insert(Decimal::ZERO) += note.total_amount;
        }
    }
    Ok(totals)
}

pub(crate) fn overlay_state<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    document_ids: Vec<Uuid>,
) -> Result<HashMap<Uuid, (CustomerInvoiceModel, Decimal)>, ArError> {
    if document_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let overlays = OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(OverlayColumn::InvoiceId.is_in(document_ids))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    let overlay_ids: Vec<Uuid> = overlays.iter().map(|row| row.id).collect();
    let applications: Vec<ArPaymentApplicationModel> = if overlay_ids.is_empty() {
        Vec::new()
    } else {
        ApplicationEntity::find()
            .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(ApplicationColumn::LegalEntityId.is_in(legal_entities.to_vec()))
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

fn matches_filters(
    item: &Value,
    document_number: &str,
    filters: &ListFilters,
    today: NaiveDate,
) -> bool {
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
        if days_from_item(item, today) < min {
            return false;
        }
    }
    if let Some(max) = filters.days_overdue_max {
        if days_from_item(item, today) > max {
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

fn days_from_item(item: &Value, today: NaiveDate) -> i64 {
    NaiveDate::parse_from_str(item["due_date"].as_str().unwrap_or_default(), "%Y-%m-%d")
        .map(|due| days_overdue(due, today))
        .unwrap_or(0)
}

pub fn list_open_items<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    filters: &ListFilters,
) -> Result<Value, ArError> {
    let entities = resolve_scope_entities(executor, session, filters.company_id)?;
    let documents = chargeback_documents(
        executor,
        session,
        &entities,
        filters.currency_code.as_deref(),
    )?;
    let document_ids: Vec<Uuid> = documents.iter().map(|row| row.id).collect();
    let credits = credit_note_totals(executor, session, &entities, document_ids.clone())?;
    let overlays = overlay_state(executor, session, &entities, document_ids)?;
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
        if filters.status.is_none() && raw_outstanding(&facts) <= Decimal::ZERO {
            // Default view is the ledger of UNPAID balances; an explicit
            // status filter still sees PAID/WRITTEN_OFF rows.
            continue;
        }
        if matches_filters(&item, &document.document_number, filters, today) {
            items.push((document.due_date, document.document_number.clone(), item));
        }
    }
    items.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let total = items.len();
    let offset = usize::try_from(i64::from(filters.page - 1) * i64::from(filters.limit))
        .unwrap_or(usize::MAX);
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
    // The detail path binds to the caller's organization: the row's legal
    // entity must be one of the org's active entities, else NotFound (no
    // leak of whether the id exists elsewhere). This never picks an
    // arbitrary entity for a multi-entity org.
    let scope = match resolve_scope_entities(executor, session, None) {
        Ok(scope) => scope,
        Err(ArError::Policy(_)) => return Err(ArError::NotFound),
        Err(error) => return Err(error),
    };
    let document = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| {
            CHARGEBACK_TYPES.contains(&row.document_type.as_str())
                && scope.contains(&row.legal_entity_id)
        })
        .ok_or(ArError::NotFound)?;
    let entity_scope = [document.legal_entity_id];
    let credits = credit_note_totals(executor, session, &entity_scope, vec![id])?;
    let overlays = overlay_state(executor, session, &entity_scope, vec![id])?;
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
        .map(|(overlay, _)| application_rows(executor, session, overlay.id, id))
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
    document_id: Uuid,
) -> Result<Vec<Value>, ArError> {
    ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::InvoiceId.eq(overlay_id))
        .all(executor)
        .map(|rows| {
            rows.into_iter()
                // A reversed pair row (applied 0) leaves the history view;
                // the immutable event journal keeps the full story.
                .filter(|row| row.applied_amount > rust_decimal::Decimal::ZERO)
                .map(|row| {
                    json!({
                        "id": row.id.to_string(),
                        "payment_id": row.payment_id.to_string(),
                        // Public contract has no overlay id: echo the
                        // posted document id, matching the item's invoice_id.
                        "customer_invoice_id": document_id.to_string(),
                        "applied_amount": money(row.applied_amount),
                    })
                })
                .collect()
        })
        .map_err(|error| ArError::Database(error.to_string()))
}

// ─── Epic F3a: fact-derived, as-of reproducible aging summary ───────────────
//
// The summary answers the SAME question as the open-items list — net
// outstanding per aging bucket — but aggregated, and reproducible for a
// past `as_of_date`. To be reproducible it reads ONLY posted facts:
// documents and credit notes posted before the cutoff, and the
// application EVENT journal (net APPLIED-REVERSED by occurred_at). It
// never reads the overlay cache columns (`status`, `original_amount`)
// and never derives from `overlay_state`; reversal rewrites the row's
// applied amount in place, so only the journal can replay history.
// `write_off_amount` has no history (manual adjustments carry no
// effective date), so a write-off counts at EVERY as-of date — stated
// here so the read is honest about its one non-temporal input.

/// Upper bound for journal events loaded by a single summary read.
const MAX_AGE_EVENTS: i64 = 100_000;
/// Upper bound for application rows loaded by a single summary read.
const MAX_AGE_APPLICATIONS: i64 = 100_000;
/// `IN`-list chunk size: far below the driver's 16-bit bind-parameter limit.
const ID_CHUNK: usize = 10_000;

/// Parse the optional `as_of_date` calendar parameter (UTC); absent = today.
pub(crate) fn parse_as_of_date(value: Option<&str>) -> Result<NaiveDate, ArError> {
    match value.map(str::trim).filter(|raw| !raw.is_empty()) {
        None => Ok(Utc::now().date_naive()),
        Some(raw) => NaiveDate::parse_from_str(raw, "%Y-%m-%d")
            .map_err(|_| ArError::Validation("as_of_date must be YYYY-MM-DD".to_string())),
    }
}

/// Contract query shape: every parameter is an optional string.
pub fn parse_aging_query(
    company_id: Option<&str>,
    customer_id: Option<&str>,
    as_of_date: Option<&str>,
) -> Result<(Option<Uuid>, Option<Uuid>, NaiveDate), ArError> {
    let uuid = |value: &str, field: &str| -> Result<Uuid, ArError> {
        Uuid::parse_str(value).map_err(|_| ArError::Validation(format!("{field} must be a UUID")))
    };
    Ok((
        company_id
            .map(|value| uuid(value, "company_id"))
            .transpose()?,
        customer_id
            .map(|value| uuid(value, "customer_id"))
            .transpose()?,
        parse_as_of_date(as_of_date)?,
    ))
}

/// Bucket accumulator for one (company, currency) scope. Pure arithmetic.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct BucketTotals {
    current: Decimal,
    one_thirty: Decimal,
    thirty_one_sixty: Decimal,
    sixty_one_ninety: Decimal,
    ninety_plus: Decimal,
    total: Decimal,
}

impl BucketTotals {
    /// `aging_bucket` returns a closed set; the fallback arm only makes a
    /// stray name land in `90+` while still counting into the total.
    fn add(&mut self, bucket: &str, amount: Decimal) {
        match bucket {
            "CURRENT" => self.current += amount,
            "1-30" => self.one_thirty += amount,
            "31-60" => self.thirty_one_sixty += amount,
            "61-90" => self.sixty_one_ninety += amount,
            _ => self.ninety_plus += amount,
        }
        self.total += amount;
    }
}

/// Same shape as `chargeback_documents` plus the as-of cutoff and
/// currency/customer narrowing — keep the two in lockstep by intent.
fn as_of_chargeback_documents<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    currency_code: &str,
    customer_id: Option<Uuid>,
    posted_before: NaiveDateTime,
) -> Result<Vec<AccountingPostedDocumentModel>, ArError> {
    let mut query = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.is_in(CHARGEBACK_TYPES))
        .filter(DocumentColumn::CurrencyCode.eq(currency_code.to_string()))
        .filter(DocumentColumn::PostedAt.lt(posted_before));
    if let Some(customer) = customer_id {
        query = query.filter(DocumentColumn::CustomerId.eq(customer));
    }
    let documents = query
        .limit((MAX_SOURCE_DOCUMENTS + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if documents.len() as i64 > MAX_SOURCE_DOCUMENTS {
        return Err(ArError::Validation(format!(
            "aging source exceeds the {MAX_SOURCE_DOCUMENTS}-document read bound; narrow the scope"
        )));
    }
    Ok(documents)
}

/// `credit_note_totals` with the same as-of cutoff on the NOTE's posting.
fn as_of_credit_note_totals<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    document_ids: Vec<Uuid>,
    posted_before: NaiveDateTime,
) -> Result<HashMap<Uuid, Decimal>, ArError> {
    if document_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let notes = DocumentEntity::find()
        .filter(DocumentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(DocumentColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(DocumentColumn::Status.eq("POSTED".to_string()))
        .filter(DocumentColumn::DocumentType.eq("CUSTOMER_CREDIT_NOTE".to_string()))
        .filter(DocumentColumn::OriginalDocumentId.is_in(document_ids))
        .filter(DocumentColumn::PostedAt.lt(posted_before))
        .limit((MAX_CREDIT_NOTES + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if notes.len() as i64 > MAX_CREDIT_NOTES {
        return Err(ArError::Validation(format!(
            "linked credit notes exceed the {MAX_CREDIT_NOTES}-row read bound"
        )));
    }
    let mut totals: HashMap<Uuid, Decimal> = HashMap::new();
    for note in notes {
        if let Some(original) = note.original_document_id {
            *totals.entry(original).or_insert(Decimal::ZERO) += note.total_amount;
        }
    }
    Ok(totals)
}

/// Overlay rows for the write-off facts ONLY (keys and `write_off_amount`);
/// the cache columns on these rows are deliberately never read here.
fn aging_overlays<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entities: &[Uuid],
    document_ids: Vec<Uuid>,
) -> Result<Vec<CustomerInvoiceModel>, ArError> {
    if document_ids.is_empty() {
        return Ok(Vec::new());
    }
    OverlayEntity::find()
        .filter(OverlayColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(OverlayColumn::LegalEntityId.is_in(legal_entities.to_vec()))
        .filter(OverlayColumn::InvoiceId.is_in(document_ids))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))
}

/// Net applied per posted document, replayed from the immutable event
/// journal: +APPLIED, -REVERSED, `occurred_at` before the cutoff. Rows
/// cannot answer history — reversal zeroes `applied_amount` in place.
fn as_of_applied_totals<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    overlays: &[CustomerInvoiceModel],
    occurred_before: NaiveDateTime,
) -> Result<HashMap<Uuid, Decimal>, ArError> {
    if overlays.is_empty() {
        return Ok(HashMap::new());
    }
    let overlay_to_document: HashMap<Uuid, Uuid> = overlays
        .iter()
        .map(|row| (row.id, row.invoice_id))
        .collect();
    // Chunked IN lists: every statement stays far below the driver's 16-bit
    // bind-parameter limit regardless of how many overlays the scope holds.
    let mut application_to_document: HashMap<Uuid, Uuid> = HashMap::new();
    let overlay_ids: Vec<Uuid> = overlay_to_document.keys().copied().collect();
    for chunk in overlay_ids.chunks(ID_CHUNK) {
        let applications = ApplicationEntity::find()
            .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(ApplicationColumn::InvoiceId.is_in(chunk.to_vec()))
            .all(executor)
            .map_err(|error| ArError::Database(error.to_string()))?;
        for row in applications {
            if let Some(document_id) = overlay_to_document.get(&row.invoice_id) {
                application_to_document.insert(row.id, *document_id);
            }
        }
        if application_to_document.len() as i64 > MAX_AGE_APPLICATIONS {
            return Err(ArError::Validation(format!(
                "application rows exceed the {MAX_AGE_APPLICATIONS}-row read bound; narrow the scope"
            )));
        }
    }
    if application_to_document.is_empty() {
        return Ok(HashMap::new());
    }
    let mut totals: HashMap<Uuid, Decimal> = HashMap::new();
    let mut seen_events: i64 = 0;
    let application_ids: Vec<Uuid> = application_to_document.keys().copied().collect();
    for chunk in application_ids.chunks(ID_CHUNK) {
        let events = EventEntity::find()
            .filter(EventColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(EventColumn::ApplicationId.is_in(chunk.to_vec()))
            .filter(EventColumn::Kind.is_in(["APPLIED".to_string(), "REVERSED".to_string()]))
            .filter(EventColumn::OccurredAt.lt(occurred_before))
            .all(executor)
            .map_err(|error| ArError::Database(error.to_string()))?;
        seen_events += events.len() as i64;
        if seen_events > MAX_AGE_EVENTS {
            return Err(ArError::Validation(format!(
                "application events exceed the {MAX_AGE_EVENTS}-row read bound; narrow the scope"
            )));
        }
        for event in &events {
            let Some(document_id) = application_to_document.get(&event.application_id) else {
                continue;
            };
            let delta = match event.kind.as_str() {
                "APPLIED" => event.amount,
                _ => -event.amount,
            };
            *totals.entry(*document_id).or_insert(Decimal::ZERO) += delta;
        }
    }
    Ok(totals)
}

/// Aggregated open-item balance for one company and its base currency.
pub fn aging_summary<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    company_id: Option<Uuid>,
    customer_id: Option<Uuid>,
    as_of: NaiveDate,
) -> Result<Value, ArError> {
    let entities = resolve_scope_entities(executor, session, company_id)?;
    // One response row can only carry one company and one currency, so an
    // ambiguous org must say which company (never an arbitrary pick).
    let company = match company_id {
        Some(id) => id,
        None if entities.len() == 1 => entities[0],
        None => {
            return Err(ArError::Validation(format!(
                "company_id is required when the organization has {} active legal entities",
                entities.len()
            )))
        }
    };
    let entity = LegalEntityEntity::find()
        .filter(LegalEntityColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(LegalEntityColumn::Id.eq(company))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or(ArError::NotFound)?;
    // Full-day cutoff: everything strictly before the next UTC midnight.
    let cutoff = as_of
        .succ_opt()
        .and_then(|next| next.and_hms_opt(0, 0, 0))
        .ok_or_else(|| ArError::Validation("as_of_date is out of range".to_string()))?;
    let base_currency = entity.base_currency;
    let documents = as_of_chargeback_documents(
        executor,
        session,
        &entities,
        &base_currency,
        customer_id,
        cutoff,
    )?;
    let document_ids: Vec<Uuid> = documents.iter().map(|row| row.id).collect();
    let credits =
        as_of_credit_note_totals(executor, session, &entities, document_ids.clone(), cutoff)?;
    let overlays = aging_overlays(executor, session, &entities, document_ids.clone())?;
    let write_offs: HashMap<Uuid, Decimal> = overlays
        .iter()
        .map(|row| {
            (
                row.invoice_id,
                row.write_off_amount.unwrap_or(Decimal::ZERO),
            )
        })
        .collect();
    let applied = as_of_applied_totals(executor, session, &overlays, cutoff)?;

    let mut totals = BucketTotals::default();
    for document in &documents {
        let raw = document.total_amount
            - credits.get(&document.id).copied().unwrap_or(Decimal::ZERO)
            - applied.get(&document.id).copied().unwrap_or(Decimal::ZERO)
            - write_offs
                .get(&document.id)
                .copied()
                .unwrap_or(Decimal::ZERO);
        if raw <= Decimal::ZERO {
            // Settled rows leave the balance view, mirroring open items.
            continue;
        }
        totals.add(aging_bucket(document.due_date, as_of), raw);
    }
    Ok(json!({
        "company_id": company.to_string(),
        "currency_code": base_currency,
        "as_of_date": as_of.format("%Y-%m-%d").to_string(),
        "current": money(totals.current),
        "bucket_1_30": money(totals.one_thirty),
        "bucket_31_60": money(totals.thirty_one_sixty),
        "bucket_61_90": money(totals.sixty_one_ninety),
        "bucket_90_plus": money(totals.ninety_plus),
        "total_outstanding": money(totals.total),
    }))
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
    fn credit_only_netting_is_partial_not_confirmed() {
        // UC-AR-08: a linked credit note counts as settlement.
        assert_eq!(
            derive_status(None, Decimal::new(40, 0), Decimal::new(20, 0)),
            "PARTIAL"
        );
        assert_eq!(
            derive_status(None, Decimal::new(40, 0), Decimal::ZERO),
            "CONFIRMED"
        );
        assert_eq!(
            derive_status(None, Decimal::ZERO, Decimal::new(120, 0)),
            "PAID"
        );
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

    #[test]
    fn bucket_totals_map_closed_bucket_set_and_total() {
        let mut totals = BucketTotals::default();
        totals.add("CURRENT", dec("120"));
        totals.add("1-30", dec("30"));
        totals.add("31-60", dec("45"));
        totals.add("61-90", dec("100"));
        totals.add("90+", dec("7"));
        assert_eq!(totals.current, dec("120"));
        assert_eq!(totals.one_thirty, dec("30"));
        assert_eq!(totals.thirty_one_sixty, dec("45"));
        assert_eq!(totals.sixty_one_ninety, dec("100"));
        assert_eq!(totals.ninety_plus, dec("7"));
        assert_eq!(totals.total, dec("302"));
    }

    #[test]
    fn bucket_names_from_aging_bucket_all_land_correctly() {
        // The summary must bucket EXACTLY like the per-item read axis.
        let due = NaiveDate::from_ymd_opt(2026, 1, 15).expect("due date");
        let probes: [(i64, &str); 6] = [
            (-5, "CURRENT"),
            (0, "CURRENT"),
            (1, "1-30"),
            (31, "31-60"),
            (62, "61-90"),
            (500, "90+"),
        ];
        for (offset, expected) in probes {
            let today = due + TimeDelta::days(offset);
            assert_eq!(aging_bucket(due, today), expected);
            let mut totals = BucketTotals::default();
            totals.add(aging_bucket(due, today), dec("10"));
            match expected {
                "CURRENT" => assert_eq!(totals.current, dec("10")),
                "1-30" => assert_eq!(totals.one_thirty, dec("10")),
                "31-60" => assert_eq!(totals.thirty_one_sixty, dec("10")),
                "61-90" => assert_eq!(totals.sixty_one_ninety, dec("10")),
                _ => assert_eq!(totals.ninety_plus, dec("10")),
            }
        }
    }

    #[test]
    fn parse_aging_query_validates_shapes() {
        // Bound the default date between reads taken around the call so a UTC
        // midnight rollover between the clock read and the parse cannot flip
        // an exact equality.
        let before = Utc::now().date_naive();
        let (company, customer, as_of) =
            parse_aging_query(None, None, None).expect("all-optional defaults");
        let after = Utc::now().date_naive();
        assert_eq!(company, None);
        assert_eq!(customer, None);
        assert!(as_of >= before && as_of <= after, "default is today");

        let company_text = Uuid::new_v4().to_string();
        let customer_text = Uuid::new_v4().to_string();
        let (company, customer, as_of) = parse_aging_query(
            Some(company_text.as_str()),
            Some(customer_text.as_str()),
            Some("2026-08-15"),
        )
        .expect("full query");
        assert_eq!(company, Some(Uuid::parse_str(&company_text).expect("uuid")));
        assert_eq!(
            customer,
            Some(Uuid::parse_str(&customer_text).expect("uuid"))
        );
        assert_eq!(as_of, NaiveDate::from_ymd_opt(2026, 8, 15).expect("date"));

        assert!(parse_aging_query(Some("not-a-uuid"), None, None).is_err());
        assert!(parse_aging_query(None, Some("not-a-uuid"), None).is_err());
        // The shipped spec example defect shape: a float where a date belongs.
        assert!(matches!(
            parse_aging_query(None, None, Some("1000.0")),
            Err(ArError::Validation(_))
        ));
    }

    fn dec(text: &str) -> Decimal {
        text.parse::<Decimal>().expect("decimal literal")
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
    #[ignore = "requires the disposable accounting DB (migrations 0001..0006) and psql on PATH; run live tests serially (--test-threads=1)"]
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
        let lone = uid("3", &run);
        let lone_credit = uid("2", &run);
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
     '{subject}'::uuid, '{mixed}'),
    ('{lone}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-3',
     'CUSTOMER_INVOICE', '{payment}', 'ledger-test', 'fixture', 'three',
     '2026-01-09', '2099-01-01', 'GBP', 2, 33, 0, 7, 40.000000, now(),
     '{subject}'::uuid, NULL),
    ('{lone_credit}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-2',
     'CUSTOMER_CREDIT_NOTE', '{payment}', 'ledger-test', 'fixture', 'credit2',
     '2026-01-10', '2099-01-01', 'GBP', 2, 12, 0, 3, 15.000000, now(),
     '{subject}'::uuid, '{lone}');
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
            listed["total"], 3,
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

        // Credit-only settlement with NO payment application must reach
        // PARTIAL through the compose_item wiring (not just derive_status).
        let lone_row = listed["items"]
            .as_array()
            .expect("items array")
            .iter()
            .find(|row| row["invoice_id"] == json!(lone.to_string()))
            .expect("credit-only invoice");
        assert_eq!(lone_row["status"], "PARTIAL");
        assert_eq!(lone_row["outstanding_amount"], "25");
        assert_eq!(lone_row["original_amount"], "25");

        let partial_only = ListFilters {
            status: Some("PARTIAL".to_string()),
            ..Default::default()
        };
        let filtered = crate::http_support::with_ar_transaction(&session, |executor| {
            list_open_items(executor, &session, &partial_only)
        })
        .expect("filtered list");
        assert_eq!(filtered["total"], 2, "credit-only row is PARTIAL too");

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

    /// CodeQL discipline: constructed SQL goes to psql STDIN, never a file.
    fn psql_seed_stdin(sql: &str) {
        use std::io::Write;
        use std::process::{Command, Stdio};
        let env =
            |key: &str, fallback: &str| std::env::var(key).unwrap_or_else(|_| fallback.to_string());
        let mut child = Command::new("psql")
            .env("PGHOST", env("DB_HOST", "localhost"))
            .env("PGPORT", env("DB_PORT", "5432"))
            .env("PGUSER", env("DB_USER", "rerp"))
            .env("PGPASSWORD", env("DB_PASS", ""))
            .env("PGDATABASE", env("DB_NAME", "rerp"))
            .args(["-v", "ON_ERROR_STOP=1", "-q"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("psql on PATH");
        child
            .stdin
            .take()
            .expect("psql stdin")
            .write_all(sql.as_bytes())
            .expect("pipe seed sql");
        let output = child.wait_with_output().expect("psql exit");
        assert!(
            output.status.success(),
            "seed failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn summary_session(tenant: &str, org: Uuid, subject: Uuid) -> SessionContext {
        SessionContext {
            tenant_id: tenant.to_string(),
            subject_id: subject,
            organization_id: org,
            session_id: "ar-f3a-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:ledger:read".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        }
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0009) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_aging_summary_replays_posted_facts_as_of() {
        // One org, TWO entities (E1 GBP, E2 EUR): the ambiguous default must
        // demand company_id, never pick. Within E1: a clean invoice, one
        // carrying a credit note and an application that was LATER reversed
        // (row rewritten to 0 in place — only the journal can replay it),
        // a fully written-off invoice, a document posted after the early
        // cutoff, and a USD document that must never join the GBP summary.
        // The write-off invoice's overlay is ALSO a poisoned cache (garbage
        // status/original_amount): the summary must ignore those columns.
        let run = run_hex();
        let tenant = format!("ar-f3a-{run}");
        let org = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let e1 = Uuid::new_v4();
        let e2 = Uuid::new_v4();
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let customer = Uuid::new_v4();
        let stranger = Uuid::new_v4();
        let doc_p = Uuid::new_v4();
        let doc_q = Uuid::new_v4();
        let note_q = Uuid::new_v4();
        let doc_r = Uuid::new_v4();
        let doc_t = Uuid::new_v4();
        let doc_u = Uuid::new_v4();
        let overlay_q = Uuid::new_v4();
        let overlay_p = Uuid::new_v4();
        let overlay_u = Uuid::new_v4();
        let payment = Uuid::new_v4();
        let application = Uuid::new_v4();
        let event_applied = Uuid::new_v4();
        let event_reversed = Uuid::new_v4();
        let doc_s2 = Uuid::new_v4();
        let tenant_b = format!("ar-f3b-{run}");

        psql_seed_stdin(&format!(
            "BEGIN;
SET search_path TO rerp, public;
SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
    'ar-f3a-test', '[\"billing\"]'::jsonb,
    '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active,
     created_at, updated_at)
VALUES
    ('{e1}', '{tenant}', '{org}', 'F3A One', 'GBP', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state,
     created_at, updated_at)
VALUES
    ('{p1}', '{tenant}', '{e1}', 'FY2026', '2026-01-01', '2026-12-31',
     'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number,
     document_type, customer_id, source_system, source_type, source_id,
     document_date, due_date, currency_code, rounding_minor_units, subtotal,
     discount_amount, tax_amount, total_amount, posted_at, posted_by,
     original_document_id)
VALUES
    ('{doc_p}', '{tenant}', '{e1}', '{p1}', '{tenant}-INV-P',
     'CUSTOMER_INVOICE', '{customer}', 'f3a', 'fixture', 'p',
     '2026-06-01', '2026-08-31', 'GBP', 2, 100, 0, 0, 100.000000,
     '2026-06-01 12:00:00', '{subject}'::uuid, NULL),
    ('{doc_q}', '{tenant}', '{e1}', '{p1}', '{tenant}-INV-Q',
     'CUSTOMER_INVOICE', '{customer}', 'f3a', 'fixture', 'q',
     '2026-09-01', '2026-10-31', 'GBP', 2, 200, 0, 0, 200.000000,
     '2026-09-01 12:00:00', '{subject}'::uuid, NULL),
    ('{note_q}', '{tenant}', '{e1}', '{p1}', '{tenant}-CRN-Q',
     'CUSTOMER_CREDIT_NOTE', '{customer}', 'f3a', 'fixture', 'qn',
     '2026-09-05', '2026-10-31', 'GBP', 2, 20, 0, 0, 20.000000,
     '2026-09-05 12:00:00', '{subject}'::uuid, '{doc_q}'),
    ('{doc_r}', '{tenant}', '{e1}', '{p1}', '{tenant}-INV-R',
     'CUSTOMER_INVOICE', '{customer}', 'f3a', 'fixture', 'r',
     '2026-06-01', '2026-09-15', 'USD', 2, 500, 0, 0, 500.000000,
     '2026-06-01 12:00:00', '{subject}'::uuid, NULL),
    ('{doc_t}', '{tenant}', '{e1}', '{p1}', '{tenant}-INV-T',
     'CUSTOMER_INVOICE', '{customer}', 'f3a', 'fixture', 't',
     '2026-10-15', '2026-12-31', 'GBP', 2, 900, 0, 0, 900.000000,
     '2026-10-15 12:00:00', '{subject}'::uuid, NULL),
    ('{doc_u}', '{tenant}', '{e1}', '{p1}', '{tenant}-INV-U',
     'CUSTOMER_INVOICE', '{customer}', 'f3a', 'fixture', 'u',
     '2026-06-01', '2026-09-30', 'GBP', 2, 60, 0, 0, 60.000000,
     '2026-06-01 12:00:00', '{subject}'::uuid, NULL);
INSERT INTO customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, updated_at,
     status, write_off_amount, write_off_date, write_off_reason)
VALUES
    ('{overlay_q}', '{tenant}', '{e1}', '{doc_q}', '{customer}',
     180.000000, 180.000000, 'GBP', '2026-10-31', 'NET_30', now(),
     'CONFIRMED', NULL, NULL, NULL),
    ('{overlay_p}', '{tenant}', '{e1}', '{doc_p}', '{customer}',
     999999.000000, 1.000000, 'GBP', '2026-08-31', 'NET_30', now(),
     'WRITTEN_OFF', NULL, NULL, NULL),
    ('{overlay_u}', '{tenant}', '{e1}', '{doc_u}', '{customer}',
     60.000000, 0.000000, 'GBP', '2026-09-30', 'NET_30', now(),
     'WRITTEN_OFF', 60.000000, '2026-09-01', 'bad debt');
INSERT INTO ar_payments
    (id, tenant_id, legal_entity_id, customer_id, payment_number, payment_date,
     payment_amount, currency_code, payment_method, status, updated_at)
VALUES ('{payment}', '{tenant}', '{e1}', '{customer}', '{tenant}-PMT-1',
     '2026-10-20', 40.000000, 'GBP', 'WIRE', 'RECONCILED', now());
INSERT INTO ar_payment_applications
    (id, tenant_id, legal_entity_id, payment_id, invoice_id, applied_amount)
VALUES ('{application}', '{tenant}', '{e1}', '{payment}', '{overlay_q}',
     0.000000);
INSERT INTO ar_payment_application_events
    (id, tenant_id, legal_entity_id, application_id, kind, amount,
     occurred_at, reason)
VALUES
    ('{event_applied}', '{tenant}', '{e1}', '{application}', 'APPLIED',
     40.000000, '2026-10-20 10:00:00', 'receipt'),
    ('{event_reversed}', '{tenant}', '{e1}', '{application}', 'REVERSED',
     40.000000, '2026-10-25 10:00:00', 'dishonoured');
COMMIT;
BEGIN;
SELECT public.rls_set_session('{tenant_b}',
    '{subject}'::uuid, '{org}'::uuid, 'ar-f3b-test',
    '[\"billing\"]'::jsonb, '[\"accounting:invoice:write\"]'::jsonb,
    'service', 'shipper');
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active,
     created_at, updated_at)
VALUES ('{e2}', '{tenant_b}', '{org}', 'F3B One', 'EUR', true, now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state,
     created_at, updated_at)
VALUES ('{p2}', '{tenant_b}', '{e2}', 'FY2026', '2026-01-01', '2026-12-31',
     'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number,
     document_type, customer_id, source_system, source_type, source_id,
     document_date, due_date, currency_code, rounding_minor_units, subtotal,
     discount_amount, tax_amount, total_amount, posted_at, posted_by,
     original_document_id)
VALUES
    ('{doc_s2}', '{tenant_b}', '{e2}', '{p2}', '{tenant_b}-INV-S',
     'CUSTOMER_INVOICE', '{customer}', 'f3b', 'fixture', 's',
     '2026-06-01', '2026-09-15', 'EUR', 2, 700, 0, 0, 700.000000,
     '2026-06-01 12:00:00', '{subject}'::uuid, NULL);
COMMIT;"
        ));

        let session = summary_session(&tenant, org, subject);

        let ask = |company: Option<Uuid>, who: Option<Uuid>, when: NaiveDate| {
            crate::http_support::with_ar_snapshot_transaction(&session, |executor| {
                aging_summary(executor, &session, company, who, when)
            })
        };

        // One entity in this (tenant, org): the default answers silently.
        let defaulted = ask(None, None, date("2026-11-01")).expect("default");
        assert_eq!(defaulted["company_id"], e1.to_string());
        assert_eq!(defaulted["currency_code"], "GBP");
        // A company that exists in ANOTHER tenant is not this scope's to
        // report — the read refuses, it never leaks or substitutes.
        assert!(matches!(
            ask(Some(e2), None, date("2026-11-01")),
            Err(ArError::Policy(_))
        ));

        // Early cutoff: only docP posted; poisoned cache on docP ignored.
        let early = ask(Some(e1), None, date("2026-08-15")).expect("early");
        assert_eq!(early["currency_code"], "GBP");
        assert_eq!(early["current"], "100");
        assert_eq!(early["total_outstanding"], "100");
        // Reproducible: same answer for the same as_of, every time.
        let early_again = ask(Some(e1), None, date("2026-08-15")).expect("early again");
        assert_eq!(early, early_again);

        // Cutoff BETWEEN the invoice posting (09-01) and its credit note
        // posting (09-05): the note must not reduce the balance yet.
        let between = ask(Some(e1), None, date("2026-09-03")).expect("between");
        assert_eq!(between["current"], "200"); // Q, before its credit note
        assert_eq!(between["bucket_1_30"], "100"); // P overdue 3d at 09-03
        assert_eq!(between["total_outstanding"], "300");

        // Journal replay: the application (10-20) counts, its reversal
        // (10-25) does not — a row-based sum could not tell these apart.
        let mid = ask(Some(e1), None, date("2026-10-22")).expect("mid");
        assert_eq!(mid["current"], "1040"); // Q 200-20-40 + T 900
        assert_eq!(mid["bucket_31_60"], "100"); // P overdue 52d at 10-22
        assert_eq!(mid["total_outstanding"], "1140");

        // After the reversal: Q reopens to 180 in 1-30; P sits in 61-90.
        let late = ask(Some(e1), None, date("2026-11-01")).expect("late");
        assert_eq!(late["current"], "900");
        assert_eq!(late["bucket_1_30"], "180");
        assert_eq!(late["bucket_61_90"], "100");
        assert_eq!(late["total_outstanding"], "1180");
        assert_eq!(late["bucket_31_60"], "0");
        assert_eq!(late["bucket_90_plus"], "0");
        assert_eq!(late["as_of_date"], "2026-11-01");
        assert_eq!(late["company_id"], e1.to_string());

        // Second company (its own tenant): sees only its EUR document.
        let session_b = summary_session(&tenant_b, org, subject);
        let other = crate::http_support::with_ar_snapshot_transaction(&session_b, |executor| {
            aging_summary(executor, &session_b, None, None, date("2026-11-01"))
        })
        .expect("e2 summary");
        assert_eq!(other["currency_code"], "EUR");
        assert_eq!(other["bucket_31_60"], "700");
        assert_eq!(other["total_outstanding"], "700");

        // Customer scope: the same customer's stranger-id sees nothing.
        let scoped = ask(Some(e1), Some(customer), date("2026-11-01")).expect("scoped");
        assert_eq!(scoped["total_outstanding"], "1180");
        let empty = ask(Some(e1), Some(stranger), date("2026-11-01")).expect("empty scope");
        assert_eq!(empty["total_outstanding"], "0");
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0009) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_aging_summary_matches_open_items_axis() {
        // The epic thesis for the reporting axis: aggregated summary ==
        // the open-items list (same tenant, same currency), value-for-
        // value, INCLUDING rows whose overlay cache is garbage.
        let run = run_hex();
        let tenant = format!("ar-f3i-{run}");
        let org = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity = Uuid::new_v4();
        let period = Uuid::new_v4();
        let customer = Uuid::new_v4();
        let i1 = Uuid::new_v4();
        let i2 = Uuid::new_v4();
        let i3 = Uuid::new_v4();
        let i4 = Uuid::new_v4();
        let i5 = Uuid::new_v4();
        let note2 = Uuid::new_v4();
        let o2 = Uuid::new_v4();
        let o4 = Uuid::new_v4();
        let o5 = Uuid::new_v4();
        let pay2 = Uuid::new_v4();
        let pay5 = Uuid::new_v4();
        let app2 = Uuid::new_v4();
        let app5 = Uuid::new_v4();
        let event2 = Uuid::new_v4();
        let event5 = Uuid::new_v4();
        let today = Utc::now().date_naive();
        let due_text = |offset: i64| {
            (today + TimeDelta::days(offset))
                .format("%Y-%m-%d")
                .to_string()
        };
        let due10 = due_text(10);
        let due_m15 = due_text(-15);
        let due_m70 = due_text(-70);
        let due_m40 = due_text(-40);
        let due_m5 = due_text(-5);

        psql_seed_stdin(&format!(
            "BEGIN;
SET search_path TO rerp, public;
SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
    'ar-f3i-test', '[\"billing\"]'::jsonb,
    '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active,
     created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org}', 'F3I One', 'GBP', true,
        now(), now());
INSERT INTO accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state,
     created_at, updated_at)
VALUES ('{period}', '{tenant}', '{entity}', 'FY2026', '2026-01-01',
     '2026-12-31', 'OPEN', now(), now());
INSERT INTO accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number,
     document_type, customer_id, source_system, source_type, source_id,
     document_date, due_date, currency_code, rounding_minor_units, subtotal,
     discount_amount, tax_amount, total_amount, posted_at, posted_by,
     original_document_id)
VALUES
    ('{i1}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-1',
     'CUSTOMER_INVOICE', '{customer}', 'f3i', 'fixture', 'one',
     '2026-01-05', '{due10}', 'GBP', 2, 100, 0, 20, 120.000000, now(),
     '{subject}'::uuid, NULL),
    ('{i2}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-2',
     'CUSTOMER_INVOICE', '{customer}', 'f3i', 'fixture', 'two',
     '2026-01-06', '{due_m15}', 'GBP', 2, 50, 0, 10, 60.000000, now(),
     '{subject}'::uuid, NULL),
    ('{note2}', '{tenant}', '{entity}', '{period}', '{tenant}-CRN-2',
     'CUSTOMER_CREDIT_NOTE', '{customer}', 'f3i', 'fixture', 'n2',
     '2026-01-07', '{due_m15}', 'GBP', 2, 16, 0, 4, 20.000000, now(),
     '{subject}'::uuid, '{i2}'),
    ('{i3}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-3',
     'CUSTOMER_INVOICE', '{customer}', 'f3i', 'fixture', 'three',
     '2026-01-08', '{due_m70}', 'USD', 2, 500, 0, 0, 500.000000, now(),
     '{subject}'::uuid, NULL),
    ('{i4}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-4',
     'CUSTOMER_INVOICE', '{customer}', 'f3i', 'fixture', 'four',
     '2026-01-09', '{due_m40}', 'GBP', 2, 45, 0, 0, 45.000000, now(),
     '{subject}'::uuid, NULL),
    ('{i5}', '{tenant}', '{entity}', '{period}', '{tenant}-INV-5',
     'CUSTOMER_INVOICE', '{customer}', 'f3i', 'fixture', 'five',
     '2026-01-10', '{due_m5}', 'GBP', 2, 80, 0, 0, 80.000000, now(),
     '{subject}'::uuid, NULL);
INSERT INTO customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, updated_at,
     status, write_off_amount)
VALUES
    ('{o2}', '{tenant}', '{entity}', '{i2}', '{customer}', 40.000000,
     30.000000, 'GBP', '{due_m15}', 'NET_30', now(), 'CONFIRMED', NULL),
    ('{o4}', '{tenant}', '{entity}', '{i4}', '{customer}', 999999.000000,
     1.000000, 'GBP', '{due_m40}', 'NET_30', now(), 'WRITTEN_OFF',
     NULL),
    ('{o5}', '{tenant}', '{entity}', '{i5}', '{customer}', 80.000000,
     0.000000, 'GBP', '{due_m5}', 'NET_30', now(), 'CONFIRMED', NULL);
INSERT INTO ar_payments
    (id, tenant_id, legal_entity_id, customer_id, payment_number, payment_date,
     payment_amount, currency_code, payment_method, status, updated_at)
VALUES
    ('{pay2}', '{tenant}', '{entity}', '{customer}', '{tenant}-PMT-2',
     '2026-01-15', 10.000000, 'GBP', 'WIRE', 'RECONCILED', now()),
    ('{pay5}', '{tenant}', '{entity}', '{customer}', '{tenant}-PMT-5',
     '2026-01-15', 80.000000, 'GBP', 'WIRE', 'RECONCILED', now());
INSERT INTO ar_payment_applications
    (id, tenant_id, legal_entity_id, payment_id, invoice_id, applied_amount)
VALUES
    ('{app2}', '{tenant}', '{entity}', '{pay2}', '{o2}', 10.000000),
    ('{app5}', '{tenant}', '{entity}', '{pay5}', '{o5}', 80.000000);
INSERT INTO ar_payment_application_events
    (id, tenant_id, legal_entity_id, application_id, kind, amount,
     occurred_at, reason)
VALUES
    ('{event2}', '{tenant}', '{entity}', '{app2}', 'APPLIED', 10.000000,
     now(), 'receipt'),
    ('{event5}', '{tenant}', '{entity}', '{app5}', 'APPLIED', 80.000000,
     now(), 'receipt');
COMMIT;"
        ));

        let session = summary_session(&tenant, org, subject);

        let summary = crate::http_support::with_ar_snapshot_transaction(&session, |executor| {
            aging_summary(executor, &session, Some(entity), None, today)
        })
        .expect("summary today");
        assert_eq!(summary["current"], "120");
        assert_eq!(summary["bucket_1_30"], "30");
        assert_eq!(summary["bucket_31_60"], "45");
        assert_eq!(summary["total_outstanding"], "195");

        // Same facts via the per-item axis (GBP only): identical totals.
        let filters = ListFilters {
            currency_code: Some("GBP".to_string()),
            limit: MAX_LIMIT,
            ..Default::default()
        };
        let listed = crate::http_support::with_ar_snapshot_transaction(&session, |executor| {
            list_open_items(executor, &session, &filters)
        })
        .expect("open items GBP");
        let mut item_sum = Decimal::ZERO;
        let mut item_current = Decimal::ZERO;
        let mut item_thirty = Decimal::ZERO;
        let mut item_sixty = Decimal::ZERO;
        for row in listed["items"].as_array().expect("items") {
            let outstanding: Decimal = row["outstanding_amount"]
                .as_str()
                .expect("money string")
                .parse()
                .expect("decimal");
            item_sum += outstanding;
            match row["aging_bucket"].as_str().expect("bucket") {
                "CURRENT" => item_current += outstanding,
                "1-30" => item_thirty += outstanding,
                "31-60" => item_sixty += outstanding,
                _ => {}
            }
        }
        assert_eq!(listed["total"], 3, "settled + USD rows leave the view");
        assert_eq!(summary["total_outstanding"], money(item_sum));
        assert_eq!(summary["current"], money(item_current));
        assert_eq!(summary["bucket_1_30"], money(item_thirty));
        assert_eq!(summary["bucket_31_60"], money(item_sixty));
    }
}
