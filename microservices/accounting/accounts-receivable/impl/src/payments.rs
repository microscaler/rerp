//! Customer receipts over the rebuildable AR overlay (Epic F2a).
//!
//! Payments are authoritative external facts: the external
//! `payment_reference` (check number, wire reference) is the duplicate
//! guard — unique per tenant + legal entity via migration 0007. The DB
//! status axis (`DRAFT|POSTED|RECONCILED|CANCELLED`) is internal; the
//! contract's status axis is DERIVED from allocation facts, mirroring how
//! F1b derives open-item status. Cash-side GL posting lands in Epic H,
//! so `posted_to_gl=false` and `gl_entry_id=null` are honest truths here.

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use lifeguard::{
    active_model::ActiveModelTrait, ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext,
};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::ledger::{resolve_scope_entities, ArError};
use rerp_accounting_accounts_receivable::models::ar_payment::{
    ArPaymentModel, ArPaymentRecord, Column as PaymentColumn, Entity as PaymentEntity,
};

const DEFAULT_LIMIT: i32 = 20;
const MAX_LIMIT: i32 = 100;
const MAX_PAYMENTS: i64 = 50_000;
const STATUS_ENUM: [&str; 5] = [
    "PENDING",
    "APPLIED",
    "APPLIED_PARTIAL",
    "REFUNDED",
    "FAILED",
];
const METHOD_ENUM: [&str; 8] = [
    "WIRE",
    "CHECK",
    "CREDIT_CARD",
    "ACH",
    "CASH",
    "BANK_TRANSFER",
    "PAYMENT_GATEWAY",
    "COUNTER_SERVICE",
];

fn money(value: Decimal) -> String {
    value.round_dp(6).normalize().to_string()
}

fn rfc3339(value: NaiveDateTime) -> String {
    value.and_utc().to_rfc3339()
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid, ArError> {
    Uuid::parse_str(value).map_err(|_| ArError::Validation(format!("{field} must be a UUID")))
}

fn parse_date(value: &str, field: &str) -> Result<NaiveDate, ArError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| ArError::Validation(format!("{field} must be a YYYY-MM-DD date")))
}

fn parse_amount(value: &str, field: &str) -> Result<Decimal, ArError> {
    value
        .parse::<Decimal>()
        .map_err(|_| ArError::Validation(format!("{field} must be a decimal string")))
}

/// Contract status axis, derived from the allocation facts. The DB status
/// only special-cases cancellation; `REFUNDED` is reserved for Epic H
/// (banking) and never derived here.
#[must_use]
pub fn derive_payment_status(db_status: &str, applied: Decimal, amount: Decimal) -> &'static str {
    if db_status == "CANCELLED" {
        return "FAILED";
    }
    if applied <= Decimal::ZERO {
        return "PENDING";
    }
    if applied >= amount {
        return "APPLIED";
    }
    "APPLIED_PARTIAL"
}

/// Contract `Payment` object over one stored receipt.
#[must_use]
pub fn payment_json(row: &ArPaymentModel) -> Value {
    payment_json_stamped(row, None)
}

// ---------------------------------------------------------------------------
// Command idempotency (Epic F2c) — the suite foundation table
// `accounting_idempotency_records` under GL's replay discipline: the
// fingerprint covers the tenant, entity, action, and every body field;
// replay of an identical body re-serves the stored result, key reuse with
// a different payload is a conflict.
// ---------------------------------------------------------------------------

pub(crate) const SOURCE_SYSTEM: &str = "accounts-receivable";

fn digest(parts: &[&[u8]]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in hasher.finalize() {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    output
}

fn optional_bytes(value: Option<&String>) -> &[u8] {
    value.map_or(b"", String::as_bytes)
}

fn create_fingerprint(tenant_id: &str, legal_entity_id: Uuid, parsed: &ParsedCreate) -> String {
    let company = parsed
        .company_id
        .map(|id| id.as_bytes().to_vec())
        .unwrap_or_default();
    digest(&[
        b"create_payment",
        tenant_id.as_bytes(),
        legal_entity_id.as_bytes(),
        parsed.amount.to_string().as_bytes(),
        parsed.currency_code.as_bytes(),
        parsed.method.as_bytes(),
        parsed.payment_date.to_string().as_bytes(),
        parsed.customer_id.as_bytes(),
        &company,
        optional_bytes(parsed.reference_number.as_ref()),
        optional_bytes(parsed.notes.as_ref()),
    ])
}

pub(crate) fn reverse_fingerprint(
    tenant_id: &str,
    legal_entity_id: Uuid,
    application_id: Uuid,
    reason: &str,
) -> String {
    digest(&[
        b"reverse_payment_application",
        tenant_id.as_bytes(),
        legal_entity_id.as_bytes(),
        application_id.as_bytes(),
        reason.as_bytes(),
    ])
}

pub(crate) fn find_idempotency<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    key: &str,
) -> Result<
    Option<
        rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordModel,
    >,
    ArError,
> {
    use rerp_entities::accounting::foundation::idempotency_record::{
        Column as IdempotencyColumn, Entity as IdempotencyEntity,
    };
    IdempotencyEntity::find()
        .filter(IdempotencyColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(IdempotencyColumn::LegalEntityId.eq(legal_entity_id))
        .filter(IdempotencyColumn::IdempotencyKey.eq(key))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))
}

/// Journal a completed command for replay. Public so the applications
/// module records reversals through the same discipline.
#[allow(clippy::too_many_arguments)] // one flat argument per record fact, slice style
pub(crate) fn record_completed<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    key: &str,
    fingerprint: &str,
    source_type: &str,
    subject_id: Uuid,
    now: NaiveDateTime,
) -> Result<(), ArError> {
    use rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordRecord;
    let mut record = AccountingIdempotencyRecordRecord::new();
    record
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_idempotency_key(key.to_string())
        .set_request_fingerprint(fingerprint.to_string())
        .set_status("COMPLETED".to_string())
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_source_type(source_type.to_string())
        .set_source_id(subject_id.to_string())
        .set_created_at(now)
        .set_completed_at(Some(now));
    record.insert(executor).map_err(|error| {
        let text = error.to_string();
        // A racing replay under the same key loses on the UNIQUE index —
        // the survivor's record is authoritative for the replay.
        if text.contains("duplicate key") {
            ArError::Conflict(
                "the idempotency key was already used with a different request body".to_string(),
            )
        } else {
            ArError::Database(text)
        }
    })?;
    Ok(())
}

/// Replay an identical body; a fingerprint mismatch is a key conflict.
/// The re-served payment still passes the read scope (invisible -> 404).
pub(crate) fn replay_payment<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    record: &rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordModel,
    fingerprint: &str,
) -> Result<Value, ArError> {
    if record.request_fingerprint != fingerprint {
        return Err(ArError::Conflict(
            "the idempotency key was already used with a different request body".to_string(),
        ));
    }
    let payment_id = Uuid::parse_str(&record.source_id).map_err(|_| {
        ArError::Database("idempotency record carries a bad payment id".to_string())
    })?;
    let row = PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::Id.eq(payment_id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or(ArError::NotFound)?;
    let stamps = last_applied_stamps(executor, session, &[row.id])?;
    Ok(payment_json_stamped(&row, stamps.get(&row.id).copied()))
}

/// `applied_at` is the last time an allocation touched this receipt
/// (F2b applications); the read axis derives it rather than storing it.
pub fn payment_json_stamped(row: &ArPaymentModel, applied_at: Option<NaiveDateTime>) -> Value {
    json!({
        "id": row.id.to_string(),
        "reference_number": row.payment_reference.clone(),
        "amount": money(row.payment_amount),
        "currency_code": row.currency_code,
        "payment_method": row.payment_method,
        "payment_date": row.payment_date.to_string(),
        "status": derive_payment_status(&row.status, row.applied_amount, row.payment_amount),
        "customer_id": row.customer_id.to_string(),
        "company_id": row.legal_entity_id.to_string(),
        "posted_to_gl": false,
        "gl_entry_id": Value::Null,
        "notes": row.notes.clone(),
        "created_at": rfc3339(row.created_at),
        "updated_at": rfc3339(row.updated_at),
        "applied_at": applied_at.map_or(Value::Null, |stamp| json!(rfc3339(stamp))),
    })
}

/// Last application instant per receipt, one query per page/batch.
pub fn last_applied_stamps<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    payment_ids: &[Uuid],
) -> Result<std::collections::HashMap<Uuid, NaiveDateTime>, ArError> {
    use rerp_accounting_accounts_receivable::models::ar_payment_application::{
        Column as ApplicationColumn, Entity as ApplicationEntity,
    };
    let mut stamps = std::collections::HashMap::new();
    if payment_ids.is_empty() {
        return Ok(stamps);
    }
    let rows = ApplicationEntity::find()
        .filter(ApplicationColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(ApplicationColumn::PaymentId.is_in(payment_ids.iter().copied()))
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    for row in rows {
        // A reversed pair row (applied 0) is not an allocation instant.
        if row.applied_amount <= Decimal::ZERO {
            continue;
        }
        stamps
            .entry(row.payment_id)
            .and_modify(|seen| *seen = (*seen).max(row.applied_at))
            .or_insert(row.applied_at);
    }
    Ok(stamps)
}

struct ParsedCreate {
    customer_id: Uuid,
    company_id: Option<Uuid>,
    amount: Decimal,
    currency_code: String,
    method: String,
    payment_date: NaiveDate,
    reference_number: Option<String>,
    notes: Option<String>,
}

/// Validates the create request into typed fields (defense in depth —
/// the dispatcher already schema-checks enums and patterns).
#[allow(clippy::too_many_arguments)] // validated contract fields, kept explicit per slice style
fn parse_create(
    amount: &str,
    currency_code: &str,
    payment_method: &str,
    payment_date: &str,
    customer_id: &str,
    company_id: Option<&str>,
    reference_number: Option<&str>,
    notes: Option<&str>,
) -> Result<ParsedCreate, ArError> {
    let amount = parse_amount(amount, "amount")?;
    if amount <= Decimal::ZERO {
        return Err(ArError::Validation(
            "amount must be greater than zero".to_string(),
        ));
    }
    let currency_code = currency_code.to_uppercase();
    if currency_code.len() != 3 || !currency_code.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(ArError::Validation(
            "currency_code must be a 3-letter ISO code".to_string(),
        ));
    }
    let method = payment_method.to_uppercase();
    if !METHOD_ENUM.contains(&method.as_str()) {
        return Err(ArError::Validation(format!(
            "payment_method must be one of {}",
            METHOD_ENUM.join(", ")
        )));
    }
    let reference_number = reference_number
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    Ok(ParsedCreate {
        customer_id: parse_uuid(customer_id, "customer_id")?,
        company_id: company_id
            .map(|value| parse_uuid(value, "company_id"))
            .transpose()?,
        amount,
        currency_code,
        method,
        payment_date: parse_date(payment_date, "payment_date")?,
        reference_number,
        notes: notes.map(str::to_string).filter(|value| !value.is_empty()),
    })
}

/// Namespace-distinct from the GL sequence lock and the F1c projection lock.
fn payment_sequence_lock_key(legal_entity_id: Uuid, year: i32) -> i64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in b"ar-payment-sequence"
        .iter()
        .chain(legal_entity_id.as_bytes())
        .chain(&year.to_le_bytes())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash as i64
}

/// `PAY-<year>-<seq>` under a transaction-scoped advisory lock, the same
/// serialization discipline as the GL document/journal sequences.
fn allocate_payment_number<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
) -> Result<String, ArError> {
    let lock_key = payment_sequence_lock_key(legal_entity_id, date.year());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| ArError::Database(format!("payment sequence lock: {error}")))?;
    let start = NaiveDate::from_ymd_opt(date.year(), 1, 1)
        .ok_or_else(|| ArError::Validation("invalid payment year".to_string()))?;
    let end = NaiveDate::from_ymd_opt(date.year(), 12, 31)
        .ok_or_else(|| ArError::Validation("invalid payment year".to_string()))?;
    let count = PaymentEntity::find()
        .filter(PaymentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(PaymentColumn::PaymentDate.gte(start))
        .filter(PaymentColumn::PaymentDate.lte(end))
        .count()
        .one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    Ok(format!("PAY-{}-{:06}", date.year(), count + 1))
}

fn duplicate_payment_reference<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    reference: &str,
) -> Result<bool, ArError> {
    PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(PaymentColumn::PaymentReference.eq(reference))
        .find_one(executor)
        .map(|row| row.is_some())
        .map_err(|error| ArError::Database(error.to_string()))
}

// One flat argument per contract body field (codegen shape).
#[allow(clippy::too_many_arguments)]
pub fn create_payment<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    idempotency_key: &str,
    amount: &str,
    currency_code: &str,
    payment_method: &str,
    payment_date: &str,
    customer_id: &str,
    company_id: Option<&str>,
    reference_number: Option<&str>,
    notes: Option<&str>,
) -> Result<Value, ArError> {
    let parsed = parse_create(
        amount,
        currency_code,
        payment_method,
        payment_date,
        customer_id,
        company_id,
        reference_number,
        notes,
    )?;
    // The caller's organization always bounds the scope: company_id can
    // narrow the entity, never escape the org (same rule as the read path).
    // A write must land on exactly one entity — never an arbitrary one.
    let scope = match resolve_scope_entities(executor, session, parsed.company_id) {
        Ok(scope) => scope,
        Err(ArError::Policy(message)) => return Err(ArError::Validation(message)),
        Err(error) => return Err(error),
    };
    let legal_entity = match scope.as_slice() {
        [single] => *single,
        _ => {
            return Err(ArError::Validation(
                "company_id is required: the organization has multiple legal entities".to_string(),
            ))
        }
    };
    // Replay discipline before any write-side guard: an identical body
    // under a known key re-serves the stored payment, never a 409 from
    // the duplicate-reference check.
    let fingerprint = create_fingerprint(session.tenant_id.as_str(), legal_entity, &parsed);
    if let Some(record) = find_idempotency(executor, session, legal_entity, idempotency_key)? {
        return replay_payment(executor, session, &record, &fingerprint);
    }
    if let Some(reference) = &parsed.reference_number {
        if duplicate_payment_reference(executor, session, legal_entity, reference)? {
            return Err(ArError::Conflict(
                "a payment with this reference_number already exists".to_string(),
            ));
        }
    }
    let payment_number = allocate_payment_number(executor, legal_entity, parsed.payment_date)?;
    let number_probe = payment_number.clone();
    let now = Utc::now().naive_utc();
    let mut record = ArPaymentRecord::new();
    record
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity)
        .set_customer_id(parsed.customer_id)
        .set_payment_number(payment_number)
        .set_payment_date(parsed.payment_date)
        .set_payment_amount(parsed.amount)
        .set_exchange_rate(Decimal::ONE)
        .set_currency_code(parsed.currency_code)
        .set_payment_method(parsed.method)
        .set_status("POSTED".to_string())
        .set_applied_amount(Decimal::ZERO)
        .set_unapplied_amount(parsed.amount)
        .set_created_at(now)
        .set_updated_at(now)
        .set_created_by(Some(session.subject_id))
        .set_updated_by(Some(session.subject_id));
    if let Some(reference) = parsed.reference_number {
        record.set_payment_reference(Some(reference));
    }
    if let Some(notes) = parsed.notes {
        record.set_notes(Some(notes));
    }
    record.insert(executor).map_err(|error| {
        let text = error.to_string();
        // RLS + the 0007 partial unique index are the final guard; a
        // racing duplicate surfaces here, not in the pre-check.
        if text.contains("duplicate key") {
            ArError::Conflict("a payment with this reference_number already exists".to_string())
        } else {
            ArError::Database(text)
        }
    })?;
    let stored = PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::LegalEntityId.eq(legal_entity))
        .filter(PaymentColumn::PaymentNumber.eq(number_probe))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .ok_or_else(|| ArError::Database("payment vanished after insert".to_string()))?;
    record_completed(
        executor,
        session,
        legal_entity,
        idempotency_key,
        &fingerprint,
        "customer_payment",
        stored.id,
        now,
    )?;
    Ok(payment_json(&stored))
}

pub struct PaymentListFilters {
    pub status: Option<String>,
    pub payment_method: Option<String>,
    pub customer_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub search: Option<String>,
    pub page: i32,
    pub limit: i32,
}

impl Default for PaymentListFilters {
    fn default() -> Self {
        Self {
            status: None,
            payment_method: None,
            customer_id: None,
            company_id: None,
            date_from: None,
            date_to: None,
            search: None,
            page: 1,
            limit: DEFAULT_LIMIT,
        }
    }
}

/// One flat argument per contract query parameter (codegen shape).
#[allow(clippy::too_many_arguments)]
pub fn parse_payment_list_filters(
    status: Option<&str>,
    payment_method: Option<&str>,
    customer_id: Option<&str>,
    company_id: Option<&str>,
    date_from: Option<&str>,
    date_to: Option<&str>,
    search: Option<&str>,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<PaymentListFilters, ArError> {
    let status = status.map(|value| value.to_uppercase());
    if let Some(value) = &status {
        if !STATUS_ENUM.contains(&value.as_str()) {
            return Err(ArError::Validation(format!(
                "status must be one of {}",
                STATUS_ENUM.join(", ")
            )));
        }
    }
    let payment_method = payment_method.map(|value| value.to_uppercase());
    if let Some(value) = &payment_method {
        if !METHOD_ENUM.contains(&value.as_str()) {
            return Err(ArError::Validation(
                "payment_method is not a known method".to_string(),
            ));
        }
    }
    Ok(PaymentListFilters {
        status,
        payment_method,
        customer_id: customer_id
            .map(|value| parse_uuid(value, "customer_id"))
            .transpose()?,
        company_id: company_id
            .map(|value| parse_uuid(value, "company_id"))
            .transpose()?,
        date_from: date_from
            .map(|value| parse_date(value, "date_from"))
            .transpose()?,
        date_to: date_to
            .map(|value| parse_date(value, "date_to"))
            .transpose()?,
        search: search
            .map(str::trim)
            .filter(|value| !value.is_empty())
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

fn matches_payment(row: &ArPaymentModel, filters: &PaymentListFilters) -> bool {
    if let Some(status) = &filters.status {
        if derive_payment_status(&row.status, row.applied_amount, row.payment_amount) != status {
            return false;
        }
    }
    if let Some(method) = &filters.payment_method {
        if &row.payment_method != method {
            return false;
        }
    }
    if let Some(customer) = filters.customer_id {
        if row.customer_id != customer {
            return false;
        }
    }
    if let Some(from) = filters.date_from {
        if row.payment_date < from {
            return false;
        }
    }
    if let Some(to) = filters.date_to {
        if row.payment_date > to {
            return false;
        }
    }
    if let Some(needle) = &filters.search {
        let haystack = format!(
            "{} {} {}",
            row.payment_number,
            row.payment_reference.clone().unwrap_or_default(),
            row.customer_id
        )
        .to_lowercase();
        if !haystack.contains(&needle.to_lowercase()) {
            return false;
        }
    }
    true
}

pub fn list_payments<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    filters: &PaymentListFilters,
) -> Result<Value, ArError> {
    let entities = resolve_scope_entities(executor, session, filters.company_id)?;
    let mut query = PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::LegalEntityId.is_in(entities));
    if let Some(method) = &filters.payment_method {
        query = query.filter(PaymentColumn::PaymentMethod.eq(method.clone()));
    }
    if let Some(customer) = filters.customer_id {
        query = query.filter(PaymentColumn::CustomerId.eq(customer));
    }
    if let Some(from) = filters.date_from {
        query = query.filter(PaymentColumn::PaymentDate.gte(from));
    }
    if let Some(to) = filters.date_to {
        query = query.filter(PaymentColumn::PaymentDate.lte(to));
    }
    let rows = query
        .limit((MAX_PAYMENTS + 1) as u64)
        .all(executor)
        .map_err(|error| ArError::Database(error.to_string()))?;
    if rows.len() as i64 > MAX_PAYMENTS {
        return Err(ArError::Validation(
            "result set exceeds the read bound; narrow the filters".to_string(),
        ));
    }
    let mut items: Vec<(NaiveDate, String, ArPaymentModel)> = Vec::new();
    for row in &rows {
        if matches_payment(row, filters) {
            items.push((row.payment_date, row.payment_number.clone(), row.clone()));
        }
    }
    items.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    let total = items.len();
    let offset = usize::try_from(i64::from(filters.page - 1) * i64::from(filters.limit))
        .unwrap_or(usize::MAX);
    let page_rows: Vec<ArPaymentModel> = items
        .into_iter()
        .skip(offset)
        .take(usize::try_from(filters.limit).unwrap_or(usize::MAX))
        .map(|(_, _, row)| row)
        .collect();
    let page_ids: Vec<Uuid> = page_rows.iter().map(|row| row.id).collect();
    let stamps = last_applied_stamps(executor, session, &page_ids)?;
    let page_items: Vec<Value> = page_rows
        .iter()
        .map(|row| payment_json_stamped(row, stamps.get(&row.id).copied()))
        .collect();
    Ok(json!({
        "items": page_items,
        "total": total,
        "page": filters.page,
        "limit": filters.limit,
        "has_more": offset + page_items.len() < total,
    }))
}

pub fn get_payment<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    id: &str,
) -> Result<Value, ArError> {
    let id = parse_uuid(id, "id")?;
    let scope = match resolve_scope_entities(executor, session, None) {
        Ok(scope) => scope,
        Err(ArError::Policy(_)) => return Err(ArError::NotFound),
        Err(error) => return Err(error),
    };
    let row = PaymentEntity::find()
        .filter(PaymentColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PaymentColumn::Id.eq(id))
        .find_one(executor)
        .map_err(|error| ArError::Database(error.to_string()))?
        .filter(|row| scope.contains(&row.legal_entity_id))
        .ok_or(ArError::NotFound)?;
    let stamps = last_applied_stamps(executor, session, &[row.id])?;
    Ok(payment_json_stamped(&row, stamps.get(&row.id).copied()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").expect("test date")
    }

    #[test]
    fn status_axis_is_derived_from_facts() {
        assert_eq!(
            derive_payment_status("POSTED", Decimal::ZERO, Decimal::ONE),
            "PENDING"
        );
        assert_eq!(
            derive_payment_status("POSTED", Decimal::new(4, 1), Decimal::ONE),
            "APPLIED_PARTIAL"
        );
        assert_eq!(
            derive_payment_status("POSTED", Decimal::ONE, Decimal::ONE),
            "APPLIED"
        );
        assert_eq!(
            derive_payment_status("RECONCILED", Decimal::ZERO, Decimal::ONE),
            "PENDING"
        );
        assert_eq!(
            derive_payment_status("CANCELLED", Decimal::ZERO, Decimal::ONE),
            "FAILED"
        );
    }

    #[test]
    fn create_validation_rejects_garbage() {
        assert!(parse_create(
            "-1",
            "USD",
            "WIRE",
            "2026-01-10",
            &Uuid::new_v4().to_string(),
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_create(
            "0",
            "USD",
            "WIRE",
            "2026-01-10",
            &Uuid::new_v4().to_string(),
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_create(
            "10",
            "USD",
            "BITCOIN",
            "2026-01-10",
            &Uuid::new_v4().to_string(),
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_create(
            "10",
            "DOLLARS",
            "WIRE",
            "2026-01-10",
            &Uuid::new_v4().to_string(),
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_create(
            "10",
            "USD",
            "WIRE",
            "01-10-2026",
            &Uuid::new_v4().to_string(),
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_create(
            "10",
            "USD",
            "WIRE",
            "2026-01-10",
            "not-a-uuid",
            None,
            None,
            None
        )
        .is_err());
    }

    #[test]
    fn create_normalizes_optional_fields() {
        let parsed = parse_create(
            "250.50",
            "usd",
            "wire",
            "2026-01-10",
            &Uuid::new_v4().to_string(),
            None,
            Some("  "),
            Some(""),
        )
        .expect("valid");
        assert_eq!(parsed.currency_code, "USD");
        assert_eq!(parsed.method, "WIRE");
        assert_eq!(parsed.amount, Decimal::new(25050, 2));
        assert!(
            parsed.reference_number.is_none(),
            "blank reference is not a reference"
        );
        assert!(parsed.notes.is_none());
        assert_eq!(parsed.payment_date, date("2026-01-10"));
    }

    #[test]
    fn list_filter_validation() {
        assert!(parse_payment_list_filters(
            Some("BOGUS"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_payment_list_filters(
            None,
            Some("CASHAPP"),
            None,
            None,
            None,
            None,
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_payment_list_filters(
            None,
            None,
            None,
            None,
            Some("2026-13-01"),
            None,
            None,
            None,
            None
        )
        .is_err());
        assert!(parse_payment_list_filters(
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
        assert!(parse_payment_list_filters(
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
        let filters = parse_payment_list_filters(
            Some("applied_partial"),
            Some("check"),
            None,
            None,
            Some("2026-01-01"),
            Some("2026-01-31"),
            Some(" REF-1 "),
            Some(2),
            Some(50),
        )
        .expect("valid filters");
        assert_eq!(filters.status.as_deref(), Some("APPLIED_PARTIAL"));
        assert_eq!(filters.payment_method.as_deref(), Some("CHECK"));
        assert_eq!(filters.search.as_deref(), Some("REF-1"));
        assert_eq!((filters.page, filters.limit), (2, 50));
    }

    #[test]
    fn sequence_lock_key_is_namespace_and_year_stable() {
        let entity = Uuid::new_v4();
        assert_eq!(
            payment_sequence_lock_key(entity, 2026),
            payment_sequence_lock_key(entity, 2026)
        );
        assert_ne!(
            payment_sequence_lock_key(entity, 2026),
            payment_sequence_lock_key(entity, 2027)
        );
        // Distinct namespace from the F1c overlay projection lock.
        let mut overlay_hash = 0xcbf2_9ce4_8422_2325_u64;
        for byte in b"ar-overlay-projection".iter().chain(entity.as_bytes()) {
            overlay_hash ^= u64::from(*byte);
            overlay_hash = overlay_hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        assert_ne!(payment_sequence_lock_key(entity, 2026), overlay_hash as i64);
    }

    // ---- live acceptance (disposable accounting DB, migrations 0001..0007) ----

    fn run_hex() -> String {
        static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        TAG.get_or_init(|| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            // Salted against the ledger and projection suites: all three
            // share one binary and the run tags must differ.
            let mix = (now ^ ((std::process::id() as u64) << 32))
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ 0xD0D0_F00D;
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
    /// file); same session shape as the projection suite.
    fn psql_run(sql: &str) -> (bool, String, String) {
        use std::io::Write;
        use std::process::Stdio;
        let mut cmd = std::process::Command::new("psql");
        psql_env(&mut cmd);
        cmd.stdin(Stdio::piped());
        // wait_with_output() only captures what was explicitly piped.
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

    /// Reads through RLS inside the tenant GUC's session.
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
            session_id: "ar-payments-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:ledger:read".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create(
        session: &SessionContext,
        amount: &str,
        method: &str,
        date: &str,
        customer: &str,
        company: Option<&str>,
        reference: Option<&str>,
    ) -> Result<Value, ArError> {
        // Fresh idempotency key per call: this helper exercises the write
        // path, not replay (replay has its own live test).
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = format!(
            "acc-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        crate::http_support::with_ar_transaction(session, |executor| {
            create_payment(
                executor,
                session,
                &key,
                amount,
                "usd",
                method,
                date,
                customer,
                company,
                reference,
                Some("acceptance"),
            )
        })
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0007) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_receipts_are_scoped_duplicate_guarded_and_derived() {
        // Tenant A holds two ORGs (the foundation pins one entity per
        // tenant+org): org 1 owns entity1 and three receipts, org 2 owns
        // entity2 and the cross-org reuse of the same external reference —
        // legal because the 0007 index is entity-scoped, yet org-invisible
        // through the session. Tenant B must see nothing at all.
        let run = run_hex();
        let tenant = format!("ar-pay-a-{run}");
        let other = format!("ar-pay-b-{run}");
        let org1 = Uuid::new_v4();
        let org2 = Uuid::new_v4();
        let foreign_org = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity1 = uid("e1", &run);
        let entity2 = uid("e2", &run);
        let entity_b = uid("b1", &run);
        let cust1 = Uuid::new_v4().to_string();
        let cust2 = Uuid::new_v4().to_string();
        let guc1 = session_guc(&tenant, &subject.to_string(), &org1.to_string(), "ar-pay-a");
        let guc2 = session_guc(&tenant, &subject.to_string(), &org2.to_string(), "ar-pay-b");
        let guc_b = session_guc(
            &other,
            &subject.to_string(),
            &foreign_org.to_string(),
            "ar-pay-c",
        );
        let today = Utc::now().date_naive();
        let day = |offset: i64| {
            (today + chrono::TimeDelta::days(offset))
                .format("%Y-%m-%d")
                .to_string()
        };
        let year = today.year();

        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc1}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity1}', '{tenant}', '{org1}', 'Payments Entity One', 'USD', true, now(), now());
COMMIT;"
        ));
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc2}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity2}', '{tenant}', '{org2}', 'Payments Entity Two', 'USD', true, now(), now());
COMMIT;"
        ));
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc_b}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity_b}', '{other}', '{foreign_org}', 'Payments Foreign', 'USD', true, now(), now());
COMMIT;"
        ));
        let session = session_for(tenant.clone(), subject, org1);
        let sibling = session_for(tenant.clone(), subject, org2);
        let foreign = session_for(other.clone(), subject, foreign_org);
        let reference = format!("REF-{run}-7");

        // Create on entity1: derived PENDING, normalized money, honest GL.
        let p1 = create(
            &session,
            "1000.50",
            "check",
            &day(0),
            &cust1,
            Some(&entity1),
            Some(&reference),
        )
        .expect("create p1");
        assert_eq!(p1["status"], "PENDING");
        assert_eq!(p1["amount"], "1000.5");
        assert_eq!(p1["payment_method"], "CHECK");
        assert_eq!(p1["currency_code"], "USD");
        assert_eq!(p1["payment_date"], day(0));
        assert_eq!(p1["company_id"], entity1);
        assert_eq!(p1["reference_number"], reference);
        assert_eq!(p1["posted_to_gl"], false);
        assert!(p1["gl_entry_id"].is_null());
        assert!(p1["applied_at"].is_null());
        assert_eq!(p1["notes"], "acceptance");
        // payment_number is server-assigned storage, NOT contract surface.

        // The external reference is the duplicate guard in its entity…
        let dup = create(
            &session,
            "7",
            "WIRE",
            &day(0),
            &cust1,
            Some(&entity1),
            Some(&reference),
        );
        assert!(matches!(dup, Err(ArError::Conflict(_))), "dup: {dup:?}");

        // …while the SAME reference in a sibling org of the same tenant is
        // a distinct index key — accepted, and invisible to org 1.
        let p2 = create(
            &sibling,
            "50",
            "cash",
            &day(-5),
            &cust2,
            Some(&entity2),
            Some(&reference),
        )
        .expect("cross-org reference allowed");
        assert_eq!(p2["company_id"], entity2);

        // NULL references never clash (partial unique index) and the
        // per-entity year sequence advances.
        create(&session, "10", "ACH", &day(0), &cust1, Some(&entity1), None)
            .expect("first null reference");
        let p4 = create(&session, "11", "ACH", &day(0), &cust1, Some(&entity1), None)
            .expect("second null reference");
        let numbers1 = psql_session_query(
            &guc1,
            &format!(
                "SELECT string_agg(payment_number, ',' ORDER BY payment_number)
                   FROM ar_payments WHERE legal_entity_id = '{entity1}'"
            ),
        );
        assert_eq!(
            numbers1,
            format!("PAY-{year}-000001,PAY-{year}-000002,PAY-{year}-000003"),
            "per-entity year sequence, in creation order"
        );
        let numbers2 = psql_session_query(
            &guc2,
            &format!(
                "SELECT payment_number FROM ar_payments WHERE id = '{}'",
                p2["id"].as_str().unwrap_or_default()
            ),
        );
        assert_eq!(
            numbers2,
            format!("PAY-{year}-000001"),
            "entity 2 counts itself"
        );
        assert!(p4["id"].is_string());

        // get: value-for-value, then org/tenant denials (no existence leak).
        let fetched = crate::http_support::with_ar_transaction(&session, |executor| {
            get_payment(executor, &session, p1["id"].as_str().unwrap_or_default())
        })
        .expect("get p1");
        assert_eq!(fetched, p1);
        assert!(matches!(
            crate::http_support::with_ar_transaction(&session, |executor| {
                get_payment(executor, &session, "not-a-uuid")
            }),
            Err(ArError::Validation(_))
        ));
        for peek in [
            crate::http_support::with_ar_transaction(&sibling, |executor| {
                get_payment(executor, &sibling, p1["id"].as_str().unwrap_or_default())
            }),
            crate::http_support::with_ar_transaction(&foreign, |executor| {
                get_payment(executor, &foreign, p1["id"].as_str().unwrap_or_default())
            }),
        ] {
            assert!(matches!(peek, Err(ArError::NotFound)), "peek: {peek:?}");
        }

        // The contract status follows the allocation columns (F2b maintains
        // them; F2a proves the read axis derives from them).
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc2}
UPDATE ar_payments SET applied_amount = 20, unapplied_amount = 30
 WHERE payment_reference = '{reference}' AND legal_entity_id = '{entity2}';
COMMIT;"
        ));
        let partial = crate::http_support::with_ar_transaction(&sibling, |executor| {
            get_payment(executor, &sibling, p2["id"].as_str().unwrap_or_default())
        })
        .expect("get p2");
        assert_eq!(partial["status"], "APPLIED_PARTIAL");

        // List: org scope holds; filters on both axes; pagination honest.
        let list_all = crate::http_support::with_ar_transaction(&session, |executor| {
            list_payments(executor, &session, &PaymentListFilters::default())
        })
        .expect("list");
        assert_eq!(list_all["total"], 3, "org 1 sees only its own receipts");
        assert_eq!(list_all["has_more"], false);

        let partial_filters = parse_payment_list_filters(
            Some("applied_partial"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .expect("derived-axis filter");
        let list_partial = crate::http_support::with_ar_transaction(&session, |executor| {
            list_payments(executor, &session, &partial_filters)
        })
        .expect("list partial");
        assert_eq!(
            list_partial["total"], 0,
            "the partial receipt lives in org 2"
        );
        let list_partial_b = crate::http_support::with_ar_transaction(&sibling, |executor| {
            list_payments(executor, &sibling, &partial_filters)
        })
        .expect("list partial org 2");
        assert_eq!(list_partial_b["total"], 1);
        assert_eq!(list_partial_b["items"][0]["id"], p2["id"]);

        let method_filters =
            parse_payment_list_filters(None, Some("ach"), None, None, None, None, None, None, None)
                .expect("method filter");
        let list_ach = crate::http_support::with_ar_transaction(&session, |executor| {
            list_payments(executor, &session, &method_filters)
        })
        .expect("list ACH");
        assert_eq!(list_ach["total"], 2);

        let search_filters = parse_payment_list_filters(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(&reference),
            Some(1),
            Some(2),
        )
        .expect("search filter");
        let list_search = crate::http_support::with_ar_transaction(&session, |executor| {
            list_payments(executor, &session, &search_filters)
        })
        .expect("list search");
        assert_eq!(list_search["total"], 1, "org 1 sees only its own reference");

        let page_filters =
            parse_payment_list_filters(None, None, None, None, None, None, None, Some(1), Some(2))
                .expect("page filters");
        let list_page = crate::http_support::with_ar_transaction(&session, |executor| {
            list_payments(executor, &session, &page_filters)
        })
        .expect("list page 1 of 2");
        assert_eq!(list_page["items"].as_array().map(Vec::len), Some(2));
        assert_eq!(list_page["has_more"], true);
        assert_eq!(list_page["total"], 3);

        // Cross-tenant: nothing visible; an alien company_id is rejected,
        // not honored.
        let list_foreign = crate::http_support::with_ar_transaction(&foreign, |executor| {
            list_payments(executor, &foreign, &PaymentListFilters::default())
        })
        .expect("list foreign");
        assert_eq!(list_foreign["total"], 0);
        let alien = parse_payment_list_filters(
            None,
            None,
            None,
            Some(&entity_b),
            None,
            None,
            None,
            None,
            None,
        )
        .expect("alien filter parses");
        assert!(matches!(
            crate::http_support::with_ar_transaction(&session, |executor| {
                list_payments(executor, &session, &alien)
            }),
            Err(ArError::Policy(_))
        ));
    }

    #[test]
    #[ignore = "requires the disposable accounting DB (migrations 0001..0009) and psql on PATH; run live tests serially (--test-threads=1)"]
    fn live_payment_creation_is_idempotent() {
        // One tenant, one entity. The idempotency key replays an identical
        // body onto the stored payment — never a duplicate row, never a
        // duplicate-reference 409 — while key reuse with a different body
        // is a conflict. Keys are per-entity, so a sibling org may reuse
        // the same key for its own receipt.
        let run = run_hex();
        let tenant = format!("ar-idem-{run}");
        let org1 = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let entity = uid("c1", &run);
        let cust = Uuid::new_v4().to_string();
        let guc = session_guc(
            &tenant,
            &subject.to_string(),
            &org1.to_string(),
            "ar-idem-a",
        );
        psql_exec(&format!(
            "BEGIN;
SET search_path TO rerp, public;
{guc}
INSERT INTO accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{entity}', '{tenant}', '{org1}', 'Idem Entity', 'USD', true, now(), now());
COMMIT;"
        ));
        let session = session_for(tenant.clone(), subject, org1);
        let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        let key = format!("idem-{run}-k1");
        let reference = format!("IDEM-{run}");

        let create_with = |amount: &str, key: &str, reference: &str| {
            crate::http_support::with_ar_transaction(&session, |executor| {
                create_payment(
                    executor,
                    &session,
                    key,
                    amount,
                    "usd",
                    "cash",
                    &today,
                    &cust,
                    Some(&entity),
                    Some(reference),
                    Some("idempotent"),
                )
            })
        };
        let first = create_with("42", &key, &reference).expect("first create");

        // Identical replay: same payment, no second row, and the reference
        // duplicate guard never fires for the replay itself.
        let replay = create_with("42", &key, &reference).expect("replay");
        assert_eq!(replay, first, "identical body replays value-for-value");
        let rows = psql_session_query(
            &guc,
            &format!("SELECT count(*) FROM ar_payments WHERE payment_reference = '{reference}'"),
        );
        assert_eq!(rows, "1", "replay stored nothing new");

        // Key reuse with a different payload is a conflict, not a second
        // receipt, and not silent overwrite.
        assert!(
            matches!(
                create_with("43", &key, &reference),
                Err(ArError::Conflict(_))
            ),
            "fingerprint mismatch must conflict"
        );

        // A fresh key carries the same body to a real duplicate-reference
        // 409 — the replay short-circuit did not paper over the guard.
        assert!(
            matches!(
                create_with("42", &format!("idem-{run}-k2"), &reference),
                Err(ArError::Conflict(_))
            ),
            "different key, same reference: 0007 guard still rules"
        );
    }
}
