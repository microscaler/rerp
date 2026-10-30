//! Manual journal entries: balanced posting and linked reversals (Epic D3).
//!
//! Mirrors the invoice posting conventions. Journal entries are immutable:
//! a reversal creates a NEW mirrored entry and links it through the
//! original document, so both audit histories survive. Number allocation
//! reuses invoice's transaction-scoped advisory lock on (legal entity,
//! calendar year), so JE numbers are collision-free across services.

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use uuid::Uuid;

use rerp_accounting_general_ledger_gen::handlers::post_journal_entry::Request as PostRequest;
use rerp_accounting_general_ledger_gen::handlers::reverse_journal_entry::Request as ReverseRequest;
use rerp_entities::accounting::foundation::account::{
    Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::audit_event::AccountingAuditEventRecord;
use rerp_entities::accounting::foundation::fiscal_period::{
    AccountingFiscalPeriodModel as FiscalPeriodModel, Column as FiscalPeriodColumn,
    Entity as FiscalPeriodEntity,
};
use rerp_entities::accounting::foundation::journal_entry::{
    AccountingJournalEntryModel as JournalModel, AccountingJournalEntryRecord,
    Column as JournalColumn, Entity as JournalEntity,
};
use rerp_entities::accounting::foundation::journal_line::{
    AccountingJournalLineModel as JournalLineModel, AccountingJournalLineRecord,
    Column as JournalLineColumn, Entity as JournalLineEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    AccountingPostedDocumentRecord, Column as DocumentColumn, Entity as DocumentEntity,
};

use crate::accounts::{find_idempotency, hex_digest, insert, SOURCE_SYSTEM};
use crate::ledger::{currency, parse_date, parse_uuid, resolve_legal_entity, LedgerError};

const SOURCE_TYPE_MANUAL: &str = "manual_journal_entry";
const SOURCE_TYPE_REVERSAL: &str = "manual_journal_reversal";
const DOCUMENT_TYPE_MANUAL: &str = "MANUAL_JOURNAL";
const DOCUMENT_TYPE_REVERSAL: &str = "MANUAL_JOURNAL_REVERSAL";
/// Manual journal entries have no counterparty; the schema keeps
/// customer_id NOT NULL and this is the documented sentinel (no FK).
const NO_COUNTERPARTY: Uuid = Uuid::from_u128(0);

pub struct JournalChange {
    pub entry: Value,
    pub created: bool,
}

pub(crate) struct PostingIds {
    pub(crate) document_id: Uuid,
    pub(crate) journal_id: Uuid,
    pub(crate) document_number: String,
    pub(crate) entry_number: String,
}

pub(crate) struct JournalDraftLine {
    pub(crate) account_id: Uuid,
    pub(crate) side: String,
    pub(crate) amount: Decimal,
    pub(crate) description: String,
}

pub fn post_journal_entry<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &PostRequest,
) -> Result<JournalChange, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint = post_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        return replay(executor, legal_entity.id, &record, &fingerprint);
    }

    let date = parse_date(&request.entry_date, "entry_date")?;
    let currency_code = currency(&request.currency_code)?;
    let period = lock_and_revalidate_period(executor, legal_entity.id, date)?;
    let lines = validate_lines(executor, legal_entity.id, &currency_code, &request.lines)?;

    let ids = allocate_identifiers(executor, legal_entity.id, date, DOCUMENT_TYPE_MANUAL, "MJE")?;
    let now = Utc::now().naive_utc();
    persist_posting(
        executor,
        session,
        legal_entity.id,
        &ids,
        &lines,
        date,
        &currency_code,
        &period,
        now,
        &fingerprint,
        &request.idempotency_key,
        DOCUMENT_TYPE_MANUAL,
        SOURCE_TYPE_MANUAL,
        None,
        "JOURNAL_ENTRY_POSTED",
    )?;
    Ok(JournalChange {
        entry: entry_json(executor, legal_entity.id, &ids)?,
        created: true,
    })
}

pub fn reverse_journal_entry<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &ReverseRequest,
) -> Result<JournalChange, LedgerError> {
    let original_id = parse_uuid(&request.id, "id")?;
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint = reverse_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        return replay(executor, legal_entity.id, &record, &fingerprint);
    }

    let original = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity.id))
        .filter(JournalColumn::Id.eq(original_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;

    let already_reversed = DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity.id))
        .filter(DocumentColumn::OriginalDocumentId.eq(original.source_document_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .is_some();
    if already_reversed {
        return Err(LedgerError::Conflict(
            "journal_entry_reversed".to_string(),
            "this journal entry has already been reversed".to_string(),
        ));
    }

    let date = Utc::now().date_naive();
    let period = lock_and_revalidate_period(executor, legal_entity.id, date)?;
    let original_lines = JournalLineEntity::find()
        .filter(JournalLineColumn::JournalEntryId.eq(original_id))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let lines = mirror_lines(&original, &original_lines);

    let ids = allocate_identifiers(
        executor,
        legal_entity.id,
        date,
        DOCUMENT_TYPE_REVERSAL,
        "MJR",
    )?;
    let now = Utc::now().naive_utc();
    persist_posting(
        executor,
        session,
        legal_entity.id,
        &ids,
        &lines,
        date,
        &original.currency_code,
        &period,
        now,
        &fingerprint,
        &request.idempotency_key,
        DOCUMENT_TYPE_REVERSAL,
        SOURCE_TYPE_REVERSAL,
        Some(original.source_document_id),
        "JOURNAL_ENTRY_REVERSED",
    )?;
    Ok(JournalChange {
        entry: entry_json(executor, legal_entity.id, &ids)?,
        created: true,
    })
}

fn validate_lines<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    currency_code: &str,
    lines: &[rerp_accounting_general_ledger_gen::handlers::types::JournalLineInput],
) -> Result<Vec<JournalDraftLine>, LedgerError> {
    if lines.len() < 2 {
        return Err(LedgerError::Validation(
            "a journal entry requires at least two lines".to_string(),
        ));
    }
    let mut account_ids: Vec<Uuid> = Vec::with_capacity(lines.len());
    for line in lines {
        account_ids.push(parse_uuid(&line.account_id, "account_id")?);
    }
    let accounts = AccountEntity::find()
        .filter(AccountColumn::LegalEntityId.eq(legal_entity_id))
        .filter(AccountColumn::Id.is_in(account_ids.iter().copied()))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let mut debit = Decimal::ZERO;
    let mut credit = Decimal::ZERO;
    let mut validated = Vec::with_capacity(lines.len());
    for (index, line) in lines.iter().enumerate() {
        let account_id = account_ids[index];
        let account = accounts
            .iter()
            .find(|account| account.id == account_id)
            .filter(|account| account.is_active)
            .ok_or_else(|| {
                LedgerError::Validation(format!(
                    "line {} references an unknown or inactive account",
                    index + 1
                ))
            })?;
        if account.currency_code != currency_code {
            return Err(LedgerError::Validation(format!(
                "line {} account currency differs from the entry currency",
                index + 1
            )));
        }
        let amount = parse_amount(&line.amount, index + 1)?;
        match line.side.as_str() {
            "DEBIT" => debit += amount,
            "CREDIT" => credit += amount,
            other => {
                return Err(LedgerError::Validation(format!(
                    "line {other} side must be DEBIT or CREDIT"
                )))
            }
        }
        validated.push(JournalDraftLine {
            account_id,
            side: line.side.clone(),
            amount,
            description: line
                .description
                .clone()
                .unwrap_or_else(|| "Manual journal line".to_string()),
        });
    }
    if debit != credit {
        return Err(LedgerError::Validation(format!(
            "journal entry is unbalanced: debits {debit} != credits {credit}"
        )));
    }
    check_totals_within_numeric_range(debit, credit)?;
    Ok(validated)
}

fn mirror_lines(
    original: &JournalModel,
    original_lines: &[JournalLineModel],
) -> Vec<JournalDraftLine> {
    original_lines
        .iter()
        .map(|line| JournalDraftLine {
            account_id: line.account_id,
            side: if line.side == "DEBIT" {
                "CREDIT".to_string()
            } else {
                "DEBIT".to_string()
            },
            amount: line.amount,
            description: format!("Reversal of {}", original.entry_number),
        })
        .collect()
}

pub(crate) fn parse_amount(value: &str, line: usize) -> Result<Decimal, LedgerError> {
    if !is_decimal_string(value) {
        return Err(LedgerError::Validation(format!(
            "line {line} amount must be a positive decimal with 1-6 fractional digits"
        )));
    }
    let amount = Decimal::from_str(value)
        .map_err(|_| LedgerError::Validation(format!("line {line} amount is not a decimal")))?;
    if amount <= Decimal::ZERO {
        return Err(LedgerError::Validation(format!(
            "line {line} amount must be positive"
        )));
    }
    Ok(amount)
}

fn is_decimal_string(value: &str) -> bool {
    let digits = value.strip_prefix('-').unwrap_or(value);
    let Some((whole, fraction)) = digits.split_once('.') else {
        return false;
    };
    !whole.is_empty()
        && whole.len() <= 13
        && !fraction.is_empty()
        && fraction.len() <= 6
        && whole.bytes().all(|byte| byte.is_ascii_digit())
        && fraction.bytes().all(|byte| byte.is_ascii_digit())
}

pub(crate) fn posting_period<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
) -> Result<FiscalPeriodModel, LedgerError> {
    FiscalPeriodEntity::find()
        .filter(FiscalPeriodColumn::LegalEntityId.eq(legal_entity_id))
        .filter(FiscalPeriodColumn::State.is_in(["OPEN", "REOPENED"]))
        .filter(FiscalPeriodColumn::StartDate.lte(date))
        .filter(FiscalPeriodColumn::EndDate.gte(date))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or_else(|| {
            LedgerError::Conflict(
                "no_open_period".to_string(),
                "no OPEN or REOPENED fiscal period covers the posting date".to_string(),
            )
        })
}

/// Serialize the "period eligibility + posting" pair per period. The
/// transition endpoint takes the same lock before reading state, so a
/// concurrent close cannot commit between the check and the persist.
pub(crate) fn period_posting_lock<E: LifeExecutor>(
    executor: &E,
    period_id: Uuid,
) -> Result<(), LedgerError> {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in b"period-posting".iter().chain(period_id.as_bytes().iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    let lock_key = i64::from_be_bytes(hash.to_be_bytes());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| LedgerError::Database(format!("period posting lock: {error}")))?;
    Ok(())
}

/// Locate the eligible posting period, take the per-period lock, then
/// re-verify its state under that lock before any caller persists.
pub(crate) fn lock_and_revalidate_period<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
) -> Result<FiscalPeriodModel, LedgerError> {
    let period = posting_period(executor, legal_entity_id, date)?;
    period_posting_lock(executor, period.id)?;
    let locked = FiscalPeriodEntity::find()
        .filter(FiscalPeriodColumn::Id.eq(period.id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;
    if locked.state != "OPEN" && locked.state != "REOPENED" {
        return Err(LedgerError::Conflict(
            "period_not_open".to_string(),
            format!(
                "the fiscal period transitioned to {} concurrently; the posting was rejected",
                locked.state
            ),
        ));
    }
    Ok(locked)
}

/// Entry totals must fit the NUMERIC(19,6) storage: individual amounts
/// pass the pattern, but sums of many max-size amounts can overflow the
/// column and fail at insert.
pub(crate) fn check_totals_within_numeric_range(
    debit: Decimal,
    credit: Decimal,
) -> Result<(), LedgerError> {
    let limit: Decimal = Decimal::from_str("9999999999999.999999")
        .map_err(|error| LedgerError::Database(format!("numeric limit: {error}")))?;
    if debit > limit || credit > limit {
        return Err(LedgerError::Validation(format!(
            "entry totals (debits {debit}, credits {credit}) exceed the NUMERIC(19,6) limit {limit}"
        )));
    }
    Ok(())
}

pub(crate) fn allocate_identifiers<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
    document_type: &str,
    document_prefix: &str,
) -> Result<PostingIds, LedgerError> {
    // Same lock key function and namespace as the invoice service: all
    // document and journal sequences for one legal entity and calendar year
    // serialize until commit. Transaction-scoped; integer derives only from
    // trusted internal values.
    let lock_key = sequence_lock_key(legal_entity_id, date.year());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| LedgerError::Database(format!("sequence lock: {error}")))?;
    let start = NaiveDate::from_ymd_opt(date.year(), 1, 1)
        .ok_or_else(|| LedgerError::Validation("invalid posting year".to_string()))?;
    let end = NaiveDate::from_ymd_opt(date.year(), 12, 31)
        .ok_or_else(|| LedgerError::Validation("invalid posting year".to_string()))?;
    let document_count = DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(DocumentColumn::DocumentType.eq(document_type))
        .filter(DocumentColumn::DocumentDate.gte(start))
        .filter(DocumentColumn::DocumentDate.lte(end))
        .count()
        .one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let journal_count = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity_id))
        .filter(JournalColumn::EntryDate.gte(start))
        .filter(JournalColumn::EntryDate.lte(end))
        .count()
        .one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let prefix = document_prefix;
    Ok(PostingIds {
        document_id: Uuid::new_v4(),
        journal_id: Uuid::new_v4(),
        document_number: format!("{prefix}-{}-{:06}", date.year(), document_count + 1),
        entry_number: format!("JE-{}-{:06}", date.year(), journal_count + 1),
    })
}

fn sequence_lock_key(legal_entity_id: Uuid, year: i32) -> i64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = FNV_OFFSET;
    for byte in legal_entity_id
        .as_bytes()
        .iter()
        .chain(year.to_be_bytes().iter())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    i64::from_be_bytes(hash.to_be_bytes())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn persist_posting<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    ids: &PostingIds,
    lines: &[JournalDraftLine],
    date: NaiveDate,
    currency_code: &str,
    period: &FiscalPeriodModel,
    now: NaiveDateTime,
    fingerprint: &str,
    idempotency_key: &str,
    document_type: &str,
    source_type: &str,
    original_document_id: Option<Uuid>,
    audit_action: &str,
) -> Result<(), LedgerError> {
    let mut debit = Decimal::ZERO;
    let mut credit = Decimal::ZERO;
    for line in lines {
        match line.side.as_str() {
            "DEBIT" => debit += line.amount,
            _ => credit += line.amount,
        }
    }

    let mut document = AccountingPostedDocumentRecord::new();
    document
        .set_id(ids.document_id)
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_fiscal_period_id(period.id)
        .set_document_number(ids.document_number.clone())
        .set_document_type(document_type.to_string())
        .set_status("POSTED".to_string())
        .set_original_document_id(original_document_id)
        .set_customer_id(NO_COUNTERPARTY)
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_source_type(source_type.to_string())
        .set_source_id(ids.journal_id.to_string())
        .set_document_date(date)
        .set_due_date(date)
        .set_currency_code(currency_code.to_string())
        .set_rounding_minor_units(0)
        .set_subtotal(Decimal::ZERO)
        .set_discount_amount(Decimal::ZERO)
        .set_tax_amount(Decimal::ZERO)
        .set_total_amount(debit)
        .set_posted_at(now)
        .set_posted_by(session.subject_id)
        .set_created_at(now);
    insert(&mut document, executor, "posted document")?;

    let mut entry = AccountingJournalEntryRecord::new();
    entry
        .set_id(ids.journal_id)
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_fiscal_period_id(period.id)
        .set_entry_number(ids.entry_number.clone())
        .set_entry_date(date)
        .set_source_document_id(ids.document_id)
        .set_currency_code(currency_code.to_string())
        .set_total_debit(debit)
        .set_total_credit(credit)
        .set_posted_at(now)
        .set_posted_by(session.subject_id)
        .set_created_at(now);
    insert(&mut entry, executor, "journal entry")?;

    for (index, line) in lines.iter().enumerate() {
        let mut record = AccountingJournalLineRecord::new();
        record
            .set_id(Uuid::new_v4())
            .set_tenant_id(session.tenant_id.clone())
            .set_legal_entity_id(legal_entity_id)
            .set_journal_entry_id(ids.journal_id)
            .set_line_number(
                i32::try_from(index + 1)
                    .map_err(|_| LedgerError::Validation("too many journal lines".to_string()))?,
            )
            .set_account_id(line.account_id)
            .set_side(line.side.clone())
            .set_amount(line.amount)
            .set_description(line.description.clone())
            .set_created_at(now);
        insert(&mut record, executor, "journal line")?;
    }

    let mut audit = AccountingAuditEventRecord::new();
    audit
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_subject_id(session.subject_id)
        .set_action(audit_action.to_string())
        .set_document_id(ids.document_id)
        .set_original_document_id(original_document_id)
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_request_fingerprint(fingerprint.to_string())
        .set_occurred_at(now);
    insert(&mut audit, executor, "audit event")?;

    let mut idempotency =
        rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordRecord::new();
    idempotency
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_idempotency_key(idempotency_key.to_string())
        .set_request_fingerprint(fingerprint.to_string())
        .set_status("COMPLETED".to_string())
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_source_type(source_type.to_string())
        .set_source_id(ids.journal_id.to_string())
        .set_document_id(Some(ids.document_id))
        .set_journal_entry_id(Some(ids.journal_id))
        .set_created_at(now)
        .set_completed_at(Some(now));
    insert(&mut idempotency, executor, "idempotency record")
}

fn replay<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    record: &rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordModel,
    fingerprint: &str,
) -> Result<JournalChange, LedgerError> {
    if record.request_fingerprint != fingerprint {
        return Err(LedgerError::Conflict(
            "idempotency_conflict".to_string(),
            "the idempotency key was already used with a different request body".to_string(),
        ));
    }
    let journal_id = record.journal_entry_id.ok_or(LedgerError::NotFound)?;
    let entry = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity_id))
        .filter(JournalColumn::Id.eq(journal_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;
    Ok(JournalChange {
        entry: entry_json_for(executor, &entry)?,
        created: false,
    })
}

fn entry_json<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    ids: &PostingIds,
) -> Result<Value, LedgerError> {
    let entry = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity_id))
        .filter(JournalColumn::Id.eq(ids.journal_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;
    entry_json_for(executor, &entry)
}

fn entry_json_for<E: LifeExecutor>(
    executor: &E,
    entry: &JournalModel,
) -> Result<Value, LedgerError> {
    let lines = JournalLineEntity::find()
        .filter(JournalLineColumn::JournalEntryId.eq(entry.id))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let document = DocumentEntity::find()
        .filter(DocumentColumn::Id.eq(entry.source_document_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let reverses_entry_id = match &document {
        Some(document) => match document.original_document_id {
            Some(original_document_id) => JournalEntity::find()
                .filter(JournalColumn::LegalEntityId.eq(entry.legal_entity_id))
                .filter(JournalColumn::SourceDocumentId.eq(original_document_id))
                .find_one(executor)
                .map_err(|error| LedgerError::Database(error.to_string()))?
                .map(|original| original.id.to_string()),
            None => None,
        },
        None => None,
    };
    let lines_json: Vec<Value> = lines
        .iter()
        .map(|line| {
            json!({
                "line_number": line.line_number,
                "account_id": line.account_id.to_string(),
                "side": line.side,
                "amount": format!("{:.6}", line.amount),
                "description": line.description,
            })
        })
        .collect();
    Ok(json!({
        "id": entry.id.to_string(),
        "entry_number": entry.entry_number,
        "document_number": document.map(|document| document.document_number),
        "entry_date": entry.entry_date.to_string(),
        "currency_code": entry.currency_code,
        "total_debit": format!("{:.6}", entry.total_debit),
        "total_credit": format!("{:.6}", entry.total_credit),
        "reverses_entry_id": reverses_entry_id,
        "lines": lines_json,
    }))
}

fn post_fingerprint(tenant_id: &str, legal_entity_id: Uuid, request: &PostRequest) -> String {
    let mut parts: Vec<Vec<u8>> = vec![
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"POST_JOURNAL_ENTRY".to_vec(),
        request.entry_date.as_bytes().to_vec(),
        request.currency_code.as_bytes().to_vec(),
    ];
    for line in &request.lines {
        let description = line.description.as_deref().unwrap_or("");
        let joined = format!(
            "{}|{}|{}|{}",
            line.account_id.trim().to_lowercase(),
            line.side,
            line.amount,
            description
        );
        parts.push(joined.into_bytes());
    }
    digest_parts(parts)
}

fn reverse_fingerprint(tenant_id: &str, legal_entity_id: Uuid, request: &ReverseRequest) -> String {
    let mut parts: Vec<Vec<u8>> = vec![
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"REVERSE_JOURNAL_ENTRY".to_vec(),
        request.id.trim().to_lowercase().into_bytes(),
    ];
    if let Some(reason) = &request.reason {
        parts.push(reason.trim().as_bytes().to_vec());
    }
    digest_parts(parts)
}

fn digest_parts(parts: Vec<Vec<u8>>) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    hex_digest(&hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rerp_accounting_general_ledger_gen::handlers::types::JournalLineInput;

    fn line(
        account: &str,
        side: &str,
        amount: &str,
        description: Option<&str>,
    ) -> JournalLineInput {
        JournalLineInput {
            account_id: account.to_string(),
            side: side.to_string(),
            amount: amount.to_string(),
            description: description.map(str::to_string),
        }
    }

    fn post_request(lines: Vec<JournalLineInput>) -> PostRequest {
        PostRequest {
            idempotency_key: "je-post-0001".to_string(),
            entry_date: "2026-05-04".to_string(),
            currency_code: "EUR".to_string(),
            lines,
        }
    }

    #[test]
    fn decimal_strings_are_strict() {
        assert!(is_decimal_string("100.00"));
        assert!(is_decimal_string("0.1"));
        assert!(is_decimal_string("1234567890123.123456"));
        assert!(!is_decimal_string("100"), "no fractional part");
        assert!(!is_decimal_string(".5"), "no whole part");
        assert!(!is_decimal_string("1e2"), "no scientific notation");
        assert!(!is_decimal_string("100.1234567"), "max 6 fractional digits");
        assert!(
            !is_decimal_string("12345678901234.1"),
            "max 13 whole digits"
        );
        assert!(!is_decimal_string(""));
    }

    #[test]
    fn post_fingerprint_binds_line_set_and_order() {
        let entity = Uuid::new_v4();
        let account_a = Uuid::new_v4().to_string();
        let account_b = Uuid::new_v4().to_string();
        let base = post_request(vec![
            line(&account_a, "DEBIT", "10.00", None),
            line(&account_b, "CREDIT", "10.00", None),
        ]);
        let swapped = post_request(vec![
            line(&account_b, "CREDIT", "10.00", None),
            line(&account_a, "DEBIT", "10.00", None),
        ]);
        let shifted = post_request(vec![
            line(&account_a, "DEBIT", "10.01", None),
            line(&account_b, "CREDIT", "9.99", None),
        ]);
        let described = post_request(vec![
            line(&account_a, "DEBIT", "10.00", Some("office rent")),
            line(&account_b, "CREDIT", "10.00", None),
        ]);
        let fingerprint = post_fingerprint("hauliage", entity, &base);
        assert_eq!(fingerprint, post_fingerprint("hauliage", entity, &base));
        assert_ne!(fingerprint, post_fingerprint("hauliage", entity, &swapped));
        assert_ne!(fingerprint, post_fingerprint("hauliage", entity, &shifted));
        assert_ne!(
            fingerprint,
            post_fingerprint("hauliage", entity, &described)
        );
    }

    #[test]
    fn post_and_reverse_fingerprints_never_collide() {
        let entity = Uuid::new_v4();
        let entry_id = Uuid::new_v4();
        let post = post_fingerprint("hauliage", entity, &post_request(vec![]));
        let reverse = reverse_fingerprint(
            "hauliage",
            entity,
            &ReverseRequest {
                id: entry_id.to_string(),
                idempotency_key: "je-reverse-0001".to_string(),
                reason: None,
            },
        );
        let reverse_with_reason = reverse_fingerprint(
            "hauliage",
            entity,
            &ReverseRequest {
                id: entry_id.to_string(),
                idempotency_key: "je-reverse-0001".to_string(),
                reason: Some("input error".to_string()),
            },
        );
        assert_ne!(post, reverse);
        assert_ne!(reverse, reverse_with_reason);
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_post_replay_reversal_and_double_reversal() {
        let session = SessionContext {
            tenant_id: "hauliage".to_string(),
            subject_id: Uuid::parse_str("a1000001-0001-4000-8000-000000000004")
                .expect("subject UUID"),
            organization_id: Uuid::parse_str("b2000002-0002-4000-8000-000000000002")
                .expect("organization UUID"),
            session_id: "runtime-acceptance".to_string(),
            roles: vec!["finance-writer".to_string()],
            permissions: vec![
                "accounting:ledger:read".to_string(),
                "accounting:ledger:write".to_string(),
            ],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        };
        // Requires an open period covering today and active accounts; the
        // fixture mirrors the D1/D2 live acceptance database setup.
        let cash = Uuid::parse_str("c3000003-0003-4000-8000-000000000003").expect("cash account");
        let rent = Uuid::parse_str("c3000003-0003-4000-8000-000000000004").expect("rent account");
        let today = Utc::now().date_naive().to_string();
        let balanced = PostRequest {
            idempotency_key: "je-live-0001".to_string(),
            entry_date: today.clone(),
            currency_code: "EUR".to_string(),
            lines: vec![
                line(&cash.to_string(), "DEBIT", "250.00", Some("office rent")),
                line(&rent.to_string(), "CREDIT", "250.00", None),
            ],
        };
        let posted = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_journal_entry(executor, &session, &balanced)
        })
        .expect("balanced post");
        assert!(posted.created);
        assert_eq!(posted.entry["total_debit"], "250.000000");
        assert_eq!(posted.entry["total_credit"], "250.000000");
        let entry_id = posted.entry["id"].as_str().expect("entry id").to_string();

        let replay = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_journal_entry(executor, &session, &balanced)
        })
        .expect("idempotent replay");
        assert!(!replay.created);
        assert_eq!(replay.entry["id"], posted.entry["id"]);

        // Unbalanced payload under a fresh key is a validation error.
        let unbalanced = PostRequest {
            idempotency_key: "je-live-0002".to_string(),
            entry_date: today,
            currency_code: "EUR".to_string(),
            lines: vec![
                line(&cash.to_string(), "DEBIT", "250.00", None),
                line(&rent.to_string(), "CREDIT", "249.00", None),
            ],
        };
        let rejected = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_journal_entry(executor, &session, &unbalanced)
        });
        assert!(matches!(rejected, Err(LedgerError::Validation(_))));

        let reversal_request = ReverseRequest {
            id: entry_id.clone(),
            idempotency_key: "je-live-0003".to_string(),
            reason: Some("input error".to_string()),
        };
        let reversed = crate::http_support::with_accounting_transaction(&session, |executor| {
            reverse_journal_entry(executor, &session, &reversal_request)
        })
        .expect("reversal");
        assert!(reversed.created);
        assert_eq!(reversed.entry["reverses_entry_id"], posted.entry["id"]);

        let double = ReverseRequest {
            id: entry_id,
            idempotency_key: "je-live-0004".to_string(),
            reason: None,
        };
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            reverse_journal_entry(executor, &session, &double)
        });
        assert!(matches!(
            conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "journal_entry_reversed"
        ));
    }

    #[test]
    fn entry_totals_beyond_numeric_storage_are_rejected_before_insert() {
        let at_limit = Decimal::from_str("9999999999999.999999").expect("limit");
        let over_limit = Decimal::from_str("9999999999999.9999999").expect("over");
        assert!(check_totals_within_numeric_range(at_limit, at_limit).is_ok());
        assert!(check_totals_within_numeric_range(over_limit, at_limit).is_err());
        assert!(check_totals_within_numeric_range(at_limit, over_limit).is_err());
    }
}
