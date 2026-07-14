//! Transaction-local persistence for the Phase 1 invoice-to-ledger slice.

use chrono::{Datelike, NaiveDate, Utc};
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, Order, SessionContext};
use rerp_accounting_core::{
    credit_customer_invoice as build_credit, post_customer_invoice as build_invoice,
    AccountingDocumentStatus, AccountingDocumentType, AuditAction, AuditEvent, CurrencyCode,
    CustomerInvoiceInstruction, FiscalPeriodState, IdempotencyKey, InvoiceLineInput,
    InvoiceLineSnapshot, InvoiceSnapshot, JournalEntry, JournalLine, JournalSide, PostingContext,
    PostingIdentifiers, PostingPlan, RoundingPolicy, SourceReference, TaxInput,
};
use rerp_accounting_invoice_gen::handlers::credit_customer_invoice::Request as CreditRequest;
use rerp_accounting_invoice_gen::handlers::post_customer_invoice::Request as PostRequest;
use rerp_accounting_invoice_gen::handlers::types::TaxFact;
use rerp_entities::accounting::foundation::account::{
    AccountingAccountModel, Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::audit_event::AccountingAuditEventRecord;
use rerp_entities::accounting::foundation::fiscal_period::{
    AccountingFiscalPeriodModel, Column as PeriodColumn, Entity as PeriodEntity,
};
use rerp_entities::accounting::foundation::idempotency_record::{
    AccountingIdempotencyRecordModel, AccountingIdempotencyRecordRecord,
    Column as IdempotencyColumn, Entity as IdempotencyEntity,
};
use rerp_entities::accounting::foundation::journal_entry::{
    AccountingJournalEntryModel, AccountingJournalEntryRecord, Column as JournalColumn,
    Entity as JournalEntity,
};
use rerp_entities::accounting::foundation::journal_line::{
    AccountingJournalLineRecord, Column as JournalLineColumn, Entity as JournalLineEntity,
};
use rerp_entities::accounting::foundation::legal_entity::{
    AccountingLegalEntityModel, Column as LegalEntityColumn, Entity as LegalEntityEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    AccountingPostedDocument, AccountingPostedDocumentRecord, Column as DocumentColumn,
    Entity as DocumentEntity,
};
use rerp_entities::accounting::foundation::posted_document_line::{
    AccountingPostedDocumentLineModel, AccountingPostedDocumentLineRecord,
    Column as DocumentLineColumn, Entity as DocumentLineEntity,
};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

const AR_ROLE: &str = "ACCOUNTS_RECEIVABLE";
const REVENUE_ROLE: &str = "REVENUE";
const TAX_ROLE: &str = "TAX_PAYABLE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PostingError {
    Validation(String),
    Policy(String),
    Conflict,
    NotFound,
    Unavailable(String),
    Database(String),
}

impl std::fmt::Display for PostingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Policy(message) => {
                write!(formatter, "accounting policy rejected command: {message}")
            }
            Self::Conflict => {
                formatter.write_str("idempotency key conflicts with existing request")
            }
            Self::NotFound => formatter.write_str("accounting resource not found"),
            Self::Unavailable(message) => write!(formatter, "dependency unavailable: {message}"),
            Self::Database(message) => write!(formatter, "database operation failed: {message}"),
        }
    }
}

impl std::error::Error for PostingError {}

#[derive(Clone, Debug)]
pub struct StoredInvoice {
    pub snapshot: InvoiceSnapshot,
    pub rounding_minor_units: i32,
}

#[derive(Clone, Debug)]
pub struct StoredPosting {
    pub idempotency_key: String,
    pub request_fingerprint: String,
    pub invoice: StoredInvoice,
    pub journal: JournalEntry,
    pub created: bool,
}

pub fn post_customer_invoice<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &PostRequest,
) -> Result<StoredPosting, PostingError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let invoice_date = parse_date(&request.invoice_date, "invoice_date")?;
    let due_date = parse_date(&request.due_date, "due_date")?;
    let currency = CurrencyCode::try_from(request.currency_code.as_str())
        .map_err(|error| PostingError::Validation(error.to_string()))?;
    if currency.as_str() != legal_entity.base_currency {
        return Err(PostingError::Policy(
            "foreign-currency posting is not enabled in Phase 1".to_string(),
        ));
    }
    let rounding_minor_units = request.rounding_minor_units.unwrap_or(2);
    let rounding = RoundingPolicy::new(
        u32::try_from(rounding_minor_units)
            .map_err(|_| PostingError::Validation("invalid rounding_minor_units".to_string()))?,
    )
    .map_err(|error| PostingError::Validation(error.to_string()))?;
    let fiscal_period = resolve_open_period(executor, legal_entity.id, invoice_date)?;
    let receivable = resolve_control_account(executor, legal_entity.id, AR_ROLE)?;
    let revenue = resolve_control_account(executor, legal_entity.id, REVENUE_ROLE)?;
    let needs_tax_account = request.lines.iter().any(|line| line.tax.is_some());
    let tax_account = needs_tax_account
        .then(|| resolve_control_account(executor, legal_entity.id, TAX_ROLE))
        .transpose()?;

    let lines = request
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let tax = parse_tax(line.tax.clone(), tax_account.as_ref(), index + 1)?;
            Ok(InvoiceLineInput {
                description: line.description.clone(),
                quantity: parse_decimal(&line.quantity, "quantity", index + 1)?,
                unit_price: parse_decimal(&line.unit_price, "unit_price", index + 1)?,
                discount_percent: parse_decimal(
                    line.discount_percent.as_deref().unwrap_or("0"),
                    "discount_percent",
                    index + 1,
                )?,
                revenue_account_id: revenue.id,
                tax,
            })
        })
        .collect::<Result<Vec<_>, PostingError>>()?;
    let instruction = CustomerInvoiceInstruction {
        idempotency_key: IdempotencyKey::try_from(request.idempotency_key.as_str())
            .map_err(|error| PostingError::Validation(error.to_string()))?,
        source: source_reference(
            &request.source.system,
            &request.source.resource_type,
            &request.source.resource_id,
        ),
        customer_id: parse_uuid(&request.customer_id, "customer_id")?,
        invoice_date,
        due_date,
        currency,
        rounding,
        receivable_account_id: receivable.id,
        lines,
    };
    let context = posting_context(session, &legal_entity, &fiscal_period);
    let identifiers = allocate_identifiers(
        executor,
        legal_entity.id,
        invoice_date,
        AccountingDocumentType::CustomerInvoice,
    )?;
    let plan = build_invoice(&context, &identifiers, &instruction)
        .map_err(|error| PostingError::Validation(error.to_string()))?;

    complete_or_persist(executor, plan, rounding_minor_units)
}

pub fn credit_customer_invoice<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    original_document_id: Uuid,
    request: &CreditRequest,
) -> Result<StoredPosting, PostingError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let original = load_posting_plan(executor, original_document_id)?;
    if original.invoice.snapshot.legal_entity_id != legal_entity.id {
        return Err(PostingError::NotFound);
    }
    let credit_date = Utc::now().date_naive();
    let fiscal_period = resolve_open_period(executor, legal_entity.id, credit_date)?;
    let context = posting_context(session, &legal_entity, &fiscal_period);
    let identifiers = allocate_identifiers(
        executor,
        legal_entity.id,
        credit_date,
        AccountingDocumentType::CustomerCreditNote,
    )?;
    let original_plan = PostingPlan {
        idempotency_key: IdempotencyKey::try_from("loaded-original")
            .map_err(|error| PostingError::Database(error.to_string()))?,
        request_fingerprint: String::new(),
        invoice: original.invoice.snapshot,
        journal: original.journal,
        audit_event: AuditEvent {
            tenant_id: session.tenant_id.clone(),
            legal_entity_id: legal_entity.id,
            subject_id: session.subject_id,
            action: AuditAction::InvoicePosted,
            document_id: original_document_id,
            original_document_id: None,
            occurred_at: Utc::now().naive_utc(),
        },
    };
    let plan = build_credit(
        &context,
        &identifiers,
        IdempotencyKey::try_from(request.idempotency_key.as_str())
            .map_err(|error| PostingError::Validation(error.to_string()))?,
        source_reference(
            &request.source.system,
            &request.source.resource_type,
            &request.source.resource_id,
        ),
        &original_plan,
        &request.reason,
    )
    .map_err(|error| PostingError::Validation(error.to_string()))?;

    complete_or_persist(executor, plan, original.invoice.rounding_minor_units)
}

pub fn get_invoice<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<StoredInvoice, PostingError> {
    load_invoice(executor, document_id)
}

pub fn get_journal<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<JournalEntry, PostingError> {
    load_journal(executor, document_id)
}

fn complete_or_persist<E: LifeExecutor>(
    executor: &E,
    plan: PostingPlan,
    rounding_minor_units: i32,
) -> Result<StoredPosting, PostingError> {
    if let Some(existing) = find_idempotency(
        executor,
        plan.invoice.legal_entity_id,
        plan.idempotency_key.as_str(),
    )? {
        if existing.request_fingerprint != plan.request_fingerprint {
            return Err(PostingError::Conflict);
        }
        let document_id = existing.document_id.ok_or_else(|| {
            PostingError::Database("completed idempotency record has no document".to_string())
        })?;
        let mut stored = load_posting_plan(executor, document_id)?;
        stored.idempotency_key = existing.idempotency_key;
        stored.request_fingerprint = existing.request_fingerprint;
        stored.created = false;
        return Ok(stored);
    }

    persist_plan(executor, &plan, rounding_minor_units)?;
    Ok(StoredPosting {
        idempotency_key: plan.idempotency_key.as_str().to_string(),
        request_fingerprint: plan.request_fingerprint,
        invoice: StoredInvoice {
            snapshot: plan.invoice,
            rounding_minor_units,
        },
        journal: plan.journal,
        created: true,
    })
}

fn resolve_legal_entity<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
) -> Result<AccountingLegalEntityModel, PostingError> {
    required_model(
        LegalEntityEntity::find()
            .filter(LegalEntityColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(LegalEntityColumn::OrganizationId.eq(session.organization_id))
            .filter(LegalEntityColumn::IsActive.eq(true))
            .find_one(executor),
        "active legal entity",
    )
}

fn resolve_open_period<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
) -> Result<AccountingFiscalPeriodModel, PostingError> {
    required_model(
        PeriodEntity::find()
            .filter(PeriodColumn::LegalEntityId.eq(legal_entity_id))
            .filter(PeriodColumn::State.eq("OPEN"))
            .filter(PeriodColumn::StartDate.lte(date))
            .filter(PeriodColumn::EndDate.gte(date))
            .find_one(executor),
        "open fiscal period",
    )
}

fn resolve_control_account<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    role: &str,
) -> Result<AccountingAccountModel, PostingError> {
    required_model(
        AccountEntity::find()
            .filter(AccountColumn::LegalEntityId.eq(legal_entity_id))
            .filter(AccountColumn::ControlRole.eq(role))
            .filter(AccountColumn::IsActive.eq(true))
            .find_one(executor),
        role,
    )
}

fn required_model<T>(
    result: Result<Option<T>, lifeguard::LifeError>,
    resource: &str,
) -> Result<T, PostingError> {
    match result {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(PostingError::Policy(format!(
            "{resource} is not configured"
        ))),
        Err(error) => Err(PostingError::Database(error.to_string())),
    }
}

fn posting_context(
    session: &SessionContext,
    legal_entity: &AccountingLegalEntityModel,
    period: &AccountingFiscalPeriodModel,
) -> PostingContext {
    PostingContext {
        tenant_id: session.tenant_id.clone(),
        legal_entity_id: legal_entity.id,
        subject_id: session.subject_id,
        fiscal_period_id: period.id,
        fiscal_period_start: period.start_date,
        fiscal_period_end: period.end_date,
        fiscal_period_state: FiscalPeriodState::Open,
        posted_at: Utc::now().naive_utc(),
    }
}

fn allocate_identifiers<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    date: NaiveDate,
    document_type: AccountingDocumentType,
) -> Result<PostingIdentifiers, PostingError> {
    // Number allocation is derived from committed rows, so serialize all
    // document/journal sequences for one legal entity and calendar year until
    // this transaction commits or rolls back. The lock is transaction-scoped
    // and its integer is derived only from trusted internal values.
    let lock_key = sequence_lock_key(legal_entity_id, date.year());
    executor
        .query_one(&format!("SELECT pg_advisory_xact_lock({lock_key})"), &[])
        .map_err(|error| PostingError::Database(format!("sequence lock: {error}")))?;
    let start = NaiveDate::from_ymd_opt(date.year(), 1, 1)
        .ok_or_else(|| PostingError::Validation("invalid document year".to_string()))?;
    let end = NaiveDate::from_ymd_opt(date.year(), 12, 31)
        .ok_or_else(|| PostingError::Validation("invalid document year".to_string()))?;
    let document_type_name = document_type_name(document_type);
    let document_count = DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(DocumentColumn::DocumentType.eq(document_type_name))
        .filter(DocumentColumn::DocumentDate.gte(start))
        .filter(DocumentColumn::DocumentDate.lte(end))
        .count()
        .one(executor)
        .map_err(|error| PostingError::Database(error.to_string()))?;
    let journal_count = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity_id))
        .filter(JournalColumn::EntryDate.gte(start))
        .filter(JournalColumn::EntryDate.lte(end))
        .count()
        .one(executor)
        .map_err(|error| PostingError::Database(error.to_string()))?;
    let prefix = match document_type {
        AccountingDocumentType::CustomerInvoice => "INV",
        AccountingDocumentType::CustomerCreditNote => "CRN",
    };
    Ok(PostingIdentifiers {
        invoice_id: Uuid::new_v4(),
        journal_entry_id: Uuid::new_v4(),
        invoice_number: format!("{prefix}-{}-{:06}", date.year(), document_count + 1),
        journal_entry_number: format!("JE-{}-{:06}", date.year(), journal_count + 1),
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

fn find_idempotency<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    key: &str,
) -> Result<Option<AccountingIdempotencyRecordModel>, PostingError> {
    match IdempotencyEntity::find()
        .filter(IdempotencyColumn::LegalEntityId.eq(legal_entity_id))
        .filter(IdempotencyColumn::IdempotencyKey.eq(key))
        .find_one(executor)
    {
        Ok(record) => Ok(record),
        Err(error) => Err(PostingError::Database(error.to_string())),
    }
}

fn persist_plan<E: LifeExecutor>(
    executor: &E,
    plan: &PostingPlan,
    rounding_minor_units: i32,
) -> Result<(), PostingError> {
    let invoice = &plan.invoice;
    let mut document = AccountingPostedDocumentRecord::new();
    document
        .set_id(invoice.id)
        .set_tenant_id(invoice.tenant_id.clone())
        .set_legal_entity_id(invoice.legal_entity_id)
        .set_fiscal_period_id(invoice.fiscal_period_id)
        .set_document_number(invoice.document_number.clone())
        .set_document_type(document_type_name(invoice.document_type).to_string())
        .set_status("POSTED".to_string())
        .set_original_document_id(invoice.original_document_id)
        .set_customer_id(invoice.customer_id)
        .set_source_system(invoice.source.system.clone())
        .set_source_type(invoice.source.resource_type.clone())
        .set_source_id(invoice.source.resource_id.clone())
        .set_document_date(invoice.invoice_date)
        .set_due_date(invoice.due_date)
        .set_currency_code(invoice.currency.as_str().to_string())
        .set_rounding_minor_units(rounding_minor_units)
        .set_subtotal(invoice.subtotal)
        .set_discount_amount(invoice.discount_amount)
        .set_tax_amount(invoice.tax_amount)
        .set_total_amount(invoice.total_amount)
        .set_posted_at(invoice.posted_at)
        .set_posted_by(plan.audit_event.subject_id)
        .set_created_at(invoice.posted_at);
    insert(&mut document, executor, "posted document")?;

    for line in &invoice.lines {
        let mut record = AccountingPostedDocumentLineRecord::new();
        record
            .set_id(Uuid::new_v4())
            .set_tenant_id(invoice.tenant_id.clone())
            .set_legal_entity_id(invoice.legal_entity_id)
            .set_document_id(invoice.id)
            .set_line_number(line.line_number as i32)
            .set_description(line.description.clone())
            .set_quantity(line.quantity)
            .set_unit_price(line.unit_price)
            .set_discount_percent(line.discount_percent)
            .set_gross_amount(line.gross_amount)
            .set_discount_amount(line.discount_amount)
            .set_net_amount(line.net_amount)
            .set_tax_code(line.tax_code.clone())
            .set_tax_rate_percent(line.tax_rate_percent)
            .set_tax_amount(line.tax_amount)
            .set_total_amount(line.total_amount)
            .set_revenue_account_id(line.revenue_account_id)
            .set_tax_liability_account_id(line.tax_liability_account_id)
            .set_created_at(invoice.posted_at);
        insert(&mut record, executor, "posted document line")?;
    }

    let journal = &plan.journal;
    let mut entry = AccountingJournalEntryRecord::new();
    entry
        .set_id(journal.id)
        .set_tenant_id(journal.tenant_id.clone())
        .set_legal_entity_id(journal.legal_entity_id)
        .set_fiscal_period_id(journal.fiscal_period_id)
        .set_entry_number(journal.entry_number.clone())
        .set_entry_date(journal.entry_date)
        .set_source_document_id(journal.source_document_id)
        .set_currency_code(journal.currency.as_str().to_string())
        .set_total_debit(journal.total_debit)
        .set_total_credit(journal.total_credit)
        .set_posted_at(journal.posted_at)
        .set_posted_by(plan.audit_event.subject_id)
        .set_created_at(journal.posted_at);
    insert(&mut entry, executor, "journal entry")?;

    for line in &journal.lines {
        let mut record = AccountingJournalLineRecord::new();
        record
            .set_id(Uuid::new_v4())
            .set_tenant_id(journal.tenant_id.clone())
            .set_legal_entity_id(journal.legal_entity_id)
            .set_journal_entry_id(journal.id)
            .set_line_number(line.line_number as i32)
            .set_account_id(line.account_id)
            .set_side(journal_side_name(line.side).to_string())
            .set_amount(line.amount)
            .set_description(line.description.clone())
            .set_created_at(journal.posted_at);
        insert(&mut record, executor, "journal line")?;
    }

    let audit = &plan.audit_event;
    let mut audit_record = AccountingAuditEventRecord::new();
    audit_record
        .set_id(Uuid::new_v4())
        .set_tenant_id(audit.tenant_id.clone())
        .set_legal_entity_id(audit.legal_entity_id)
        .set_subject_id(audit.subject_id)
        .set_action(audit_action_name(audit.action).to_string())
        .set_document_id(audit.document_id)
        .set_original_document_id(audit.original_document_id)
        .set_source_system(invoice.source.system.clone())
        .set_request_fingerprint(plan.request_fingerprint.clone())
        .set_occurred_at(audit.occurred_at)
        .set_recorded_at(audit.occurred_at);
    insert(&mut audit_record, executor, "audit event")?;

    let mut idempotency = AccountingIdempotencyRecordRecord::new();
    idempotency
        .set_id(Uuid::new_v4())
        .set_tenant_id(invoice.tenant_id.clone())
        .set_legal_entity_id(invoice.legal_entity_id)
        .set_idempotency_key(plan.idempotency_key.as_str().to_string())
        .set_request_fingerprint(plan.request_fingerprint.clone())
        .set_status("COMPLETED".to_string())
        .set_source_system(invoice.source.system.clone())
        .set_source_type(invoice.source.resource_type.clone())
        .set_source_id(invoice.source.resource_id.clone())
        .set_document_id(Some(invoice.id))
        .set_journal_entry_id(Some(journal.id))
        .set_created_at(invoice.posted_at)
        .set_completed_at(Some(invoice.posted_at));
    insert(&mut idempotency, executor, "idempotency record")
}

fn insert<R: ActiveModelTrait, E: LifeExecutor>(
    record: &mut R,
    executor: &E,
    resource: &str,
) -> Result<(), PostingError> {
    record
        .insert(executor)
        .map(|_| ())
        .map_err(|error| PostingError::Database(format!("{resource}: {error}")))
}

fn load_posting_plan<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<StoredPosting, PostingError> {
    let invoice = load_invoice(executor, document_id)?;
    let journal = load_journal(executor, document_id)?;
    Ok(StoredPosting {
        idempotency_key: String::new(),
        request_fingerprint: String::new(),
        invoice,
        journal,
        created: false,
    })
}

fn load_invoice<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<StoredInvoice, PostingError> {
    let document = match DocumentEntity::find()
        .filter(DocumentColumn::Id.eq(document_id))
        .find_one(executor)
    {
        Ok(Some(document)) => document,
        Ok(None) => return Err(PostingError::NotFound),
        Err(error) => return Err(PostingError::Database(error.to_string())),
    };
    let lines = DocumentLineEntity::find()
        .filter(DocumentLineColumn::DocumentId.eq(document_id))
        .order_by(DocumentLineColumn::LineNumber, Order::Asc)
        .all(executor)
        .map_err(|error| PostingError::Database(error.to_string()))?;
    let snapshots = lines
        .into_iter()
        .map(document_line_snapshot)
        .collect::<Result<Vec<_>, PostingError>>()?;
    Ok(StoredInvoice {
        rounding_minor_units: document.rounding_minor_units,
        snapshot: InvoiceSnapshot {
            id: document.id,
            tenant_id: document.tenant_id,
            legal_entity_id: document.legal_entity_id,
            fiscal_period_id: document.fiscal_period_id,
            document_number: document.document_number,
            document_type: parse_document_type(&document.document_type)?,
            status: AccountingDocumentStatus::Posted,
            original_document_id: document.original_document_id,
            customer_id: document.customer_id,
            source: source_reference(
                &document.source_system,
                &document.source_type,
                &document.source_id,
            ),
            invoice_date: document.document_date,
            due_date: document.due_date,
            currency: CurrencyCode::try_from(document.currency_code)
                .map_err(|error| PostingError::Database(error.to_string()))?,
            posted_at: document.posted_at,
            subtotal: document.subtotal,
            discount_amount: document.discount_amount,
            tax_amount: document.tax_amount,
            total_amount: document.total_amount,
            lines: snapshots,
        },
    })
}

fn document_line_snapshot(
    line: AccountingPostedDocumentLineModel,
) -> Result<InvoiceLineSnapshot, PostingError> {
    Ok(InvoiceLineSnapshot {
        line_number: u32::try_from(line.line_number)
            .map_err(|_| PostingError::Database("negative invoice line number".to_string()))?,
        description: line.description,
        quantity: line.quantity,
        unit_price: line.unit_price,
        discount_percent: line.discount_percent,
        gross_amount: line.gross_amount,
        discount_amount: line.discount_amount,
        net_amount: line.net_amount,
        tax_code: line.tax_code,
        tax_rate_percent: line.tax_rate_percent,
        tax_amount: line.tax_amount,
        total_amount: line.total_amount,
        revenue_account_id: line.revenue_account_id,
        tax_liability_account_id: line.tax_liability_account_id,
    })
}

fn load_journal<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<JournalEntry, PostingError> {
    let entry: AccountingJournalEntryModel = match JournalEntity::find()
        .filter(JournalColumn::SourceDocumentId.eq(document_id))
        .find_one(executor)
    {
        Ok(Some(entry)) => entry,
        Ok(None) => return Err(PostingError::NotFound),
        Err(error) => return Err(PostingError::Database(error.to_string())),
    };
    let lines = JournalLineEntity::find()
        .filter(JournalLineColumn::JournalEntryId.eq(entry.id))
        .order_by(JournalLineColumn::LineNumber, Order::Asc)
        .all(executor)
        .map_err(|error| PostingError::Database(error.to_string()))?
        .into_iter()
        .map(|line| {
            Ok(JournalLine {
                line_number: u32::try_from(line.line_number).map_err(|_| {
                    PostingError::Database("negative journal line number".to_string())
                })?,
                account_id: line.account_id,
                description: line.description,
                side: parse_journal_side(&line.side)?,
                amount: line.amount,
            })
        })
        .collect::<Result<Vec<_>, PostingError>>()?;
    let journal = JournalEntry {
        id: entry.id,
        tenant_id: entry.tenant_id,
        legal_entity_id: entry.legal_entity_id,
        fiscal_period_id: entry.fiscal_period_id,
        entry_number: entry.entry_number,
        entry_date: entry.entry_date,
        source_document_id: entry.source_document_id,
        currency: CurrencyCode::try_from(entry.currency_code)
            .map_err(|error| PostingError::Database(error.to_string()))?,
        lines,
        posted_at: entry.posted_at,
        total_debit: entry.total_debit,
        total_credit: entry.total_credit,
    };
    journal
        .validate()
        .map_err(|error| PostingError::Database(error.to_string()))?;
    Ok(journal)
}

fn parse_tax(
    value: Option<Value>,
    account: Option<&AccountingAccountModel>,
    line_number: usize,
) -> Result<Option<TaxInput>, PostingError> {
    let Some(value) = value.filter(|value| !value.is_null()) else {
        return Ok(None);
    };
    let fact: TaxFact = serde_json::from_value(value).map_err(|_| {
        PostingError::Validation(format!("line {line_number} has an invalid tax fact"))
    })?;
    let account = account.ok_or_else(|| {
        PostingError::Policy("tax liability account is not configured".to_string())
    })?;
    Ok(Some(TaxInput {
        code: fact.code,
        rate_percent: parse_decimal(&fact.rate_percent, "tax.rate_percent", line_number)?,
        liability_account_id: account.id,
    }))
}

fn parse_decimal(value: &str, field: &str, line: usize) -> Result<Decimal, PostingError> {
    Decimal::from_str(value).map_err(|_| {
        PostingError::Validation(format!("line {line} field {field} is not a decimal"))
    })
}

fn parse_date(value: &str, field: &str) -> Result<NaiveDate, PostingError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| PostingError::Validation(format!("{field} must use YYYY-MM-DD")))
}

pub fn parse_uuid(value: &str, field: &str) -> Result<Uuid, PostingError> {
    Uuid::parse_str(value).map_err(|_| PostingError::Validation(format!("{field} must be a UUID")))
}

fn source_reference(system: &str, resource_type: &str, resource_id: &str) -> SourceReference {
    SourceReference {
        system: system.to_string(),
        resource_type: resource_type.to_string(),
        resource_id: resource_id.to_string(),
    }
}

fn document_type_name(document_type: AccountingDocumentType) -> &'static str {
    match document_type {
        AccountingDocumentType::CustomerInvoice => "CUSTOMER_INVOICE",
        AccountingDocumentType::CustomerCreditNote => "CUSTOMER_CREDIT_NOTE",
    }
}

fn parse_document_type(value: &str) -> Result<AccountingDocumentType, PostingError> {
    match value {
        "CUSTOMER_INVOICE" => Ok(AccountingDocumentType::CustomerInvoice),
        "CUSTOMER_CREDIT_NOTE" => Ok(AccountingDocumentType::CustomerCreditNote),
        _ => Err(PostingError::Database("unknown document type".to_string())),
    }
}

fn journal_side_name(side: JournalSide) -> &'static str {
    match side {
        JournalSide::Debit => "DEBIT",
        JournalSide::Credit => "CREDIT",
    }
}

fn parse_journal_side(value: &str) -> Result<JournalSide, PostingError> {
    match value {
        "DEBIT" => Ok(JournalSide::Debit),
        "CREDIT" => Ok(JournalSide::Credit),
        _ => Err(PostingError::Database("unknown journal side".to_string())),
    }
}

fn audit_action_name(action: AuditAction) -> &'static str {
    match action {
        AuditAction::InvoicePosted => "INVOICE_POSTED",
        AuditAction::CreditNotePosted => "CREDIT_NOTE_POSTED",
    }
}

pub fn invoice_json(invoice: &StoredInvoice) -> Value {
    let rounding_minor_units = invoice.rounding_minor_units;
    let invoice = &invoice.snapshot;
    json!({
        "id": invoice.id,
        "document_number": invoice.document_number,
        "document_type": document_type_name(invoice.document_type),
        "status": "POSTED",
        "original_document_id": invoice.original_document_id,
        "customer_id": invoice.customer_id,
        "source": {
            "system": invoice.source.system,
            "resource_type": invoice.source.resource_type,
            "resource_id": invoice.source.resource_id,
        },
        "invoice_date": invoice.invoice_date,
        "due_date": invoice.due_date,
        "currency_code": invoice.currency.as_str(),
        "rounding_minor_units": rounding_minor_units,
        "subtotal": invoice.subtotal.to_string(),
        "discount_amount": invoice.discount_amount.to_string(),
        "tax_amount": invoice.tax_amount.to_string(),
        "total_amount": invoice.total_amount.to_string(),
        "posted_at": invoice.posted_at.and_utc().to_rfc3339(),
        "lines": invoice.lines.iter().map(|line| json!({
            "line_number": line.line_number,
            "description": line.description,
            "quantity": line.quantity.to_string(),
            "unit_price": line.unit_price.to_string(),
            "discount_percent": line.discount_percent.to_string(),
            "gross_amount": line.gross_amount.to_string(),
            "discount_amount": line.discount_amount.to_string(),
            "net_amount": line.net_amount.to_string(),
            "tax_code": line.tax_code,
            "tax_rate_percent": line.tax_rate_percent.map(|value| value.to_string()),
            "tax_amount": line.tax_amount.to_string(),
            "total_amount": line.total_amount.to_string(),
        })).collect::<Vec<_>>(),
    })
}

pub fn journal_json(journal: &JournalEntry) -> Value {
    json!({
        "id": journal.id,
        "entry_number": journal.entry_number,
        "entry_date": journal.entry_date,
        "source_document_id": journal.source_document_id,
        "currency_code": journal.currency.as_str(),
        "total_debit": journal.total_debit.to_string(),
        "total_credit": journal.total_credit.to_string(),
        "posted_at": journal.posted_at.and_utc().to_rfc3339(),
        "lines": journal.lines.iter().map(|line| json!({
            "line_number": line.line_number,
            "account_id": line.account_id,
            "description": line.description,
            "side": journal_side_name(line.side),
            "amount": line.amount.to_string(),
        })).collect::<Vec<_>>(),
    })
}

pub fn posting_json(posting: &StoredPosting) -> Value {
    json!({
        "idempotency_key": posting.idempotency_key,
        "request_fingerprint": posting.request_fingerprint,
        "invoice": invoice_json(&posting.invoice),
        "journal": journal_json(&posting.journal),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rerp_accounting_invoice_gen::handlers::types::{CustomerInvoiceLine, SourceReference};

    #[test]
    fn public_invoice_json_does_not_expose_tenant_or_gl_mapping() {
        let value = json!({
            "tenant_id": "must-not-leak",
            "legal_entity_id": Uuid::nil(),
            "revenue_account_id": Uuid::nil()
        });
        let serialized = value.to_string();
        assert!(serialized.contains("must-not-leak"));
        assert!(!invoice_json_keys().contains(&"tenant_id"));
        assert!(!invoice_json_keys().contains(&"legal_entity_id"));
        assert!(!invoice_json_keys().contains(&"revenue_account_id"));
    }

    fn invoice_json_keys() -> Vec<&'static str> {
        vec![
            "id",
            "document_number",
            "document_type",
            "status",
            "original_document_id",
            "customer_id",
            "source",
            "invoice_date",
            "due_date",
            "currency_code",
            "rounding_minor_units",
            "subtotal",
            "discount_amount",
            "tax_amount",
            "total_amount",
            "posted_at",
            "lines",
        ]
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_post_retry_conflict_retrieve_and_credit() {
        let session = SessionContext {
            tenant_id: "hauliage".to_string(),
            subject_id: Uuid::parse_str("a1000001-0001-4000-8000-000000000004")
                .expect("subject UUID"),
            organization_id: Uuid::parse_str("b2000002-0002-4000-8000-000000000002")
                .expect("organization UUID"),
            session_id: "runtime-acceptance".to_string(),
            roles: vec!["billing".to_string()],
            permissions: vec!["accounting:invoice:write".to_string()],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        };
        let today = Utc::now().date_naive().to_string();
        let original_request = request(&today, "runtime-post-1", "100.00");

        let first = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_customer_invoice(executor, &session, &original_request)
        })
        .expect("first posting");
        assert!(first.created);
        assert_eq!(first.invoice.snapshot.total_amount.to_string(), "120.00");
        assert_eq!(first.journal.total_debit, first.journal.total_credit);

        let retry = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_customer_invoice(executor, &session, &original_request)
        })
        .expect("idempotent retry");
        assert!(!retry.created);
        assert_eq!(retry.invoice.snapshot.id, first.invoice.snapshot.id);

        let changed = request(&today, "runtime-post-1", "101.00");
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            post_customer_invoice(executor, &session, &changed)
        });
        assert!(conflict.is_err());

        let loaded = crate::http_support::with_accounting_transaction(&session, |executor| {
            get_invoice(executor, first.invoice.snapshot.id)
        })
        .expect("retrieve invoice");
        assert_eq!(
            loaded.snapshot.total_amount,
            first.invoice.snapshot.total_amount
        );

        let credit_request = CreditRequest {
            idempotency_key: "runtime-credit-1".to_string(),
            reason: "Commercial cancellation".to_string(),
            source: SourceReference {
                system: "hauliage".to_string(),
                resource_type: "delivery_credit".to_string(),
                resource_id: "delivery-acceptance-credit".to_string(),
            },
            id: first.invoice.snapshot.id.to_string(),
        };
        let credit = crate::http_support::with_accounting_transaction(&session, |executor| {
            credit_customer_invoice(
                executor,
                &session,
                first.invoice.snapshot.id,
                &credit_request,
            )
        })
        .expect("credit note");
        assert_eq!(
            credit.invoice.snapshot.original_document_id,
            Some(first.invoice.snapshot.id)
        );
        assert_eq!(credit.journal.total_debit, first.journal.total_credit);

        let concurrent = (0..2)
            .map(|index| {
                let session = session.clone();
                let date = today.clone();
                std::thread::spawn(move || {
                    let key = format!("runtime-concurrent-{index}");
                    let request = request(&date, &key, "25.00");
                    crate::http_support::with_accounting_transaction(&session, |executor| {
                        post_customer_invoice(executor, &session, &request)
                    })
                    .expect("concurrent posting")
                })
            })
            .map(|handle| handle.join().expect("concurrent posting thread"))
            .collect::<Vec<_>>();
        assert_ne!(
            concurrent[0].invoice.snapshot.document_number,
            concurrent[1].invoice.snapshot.document_number
        );
    }

    fn request(date: &str, idempotency_key: &str, unit_price: &str) -> PostRequest {
        PostRequest {
            currency_code: "GBP".to_string(),
            customer_id: "c3000003-0003-4000-8000-000000000003".to_string(),
            due_date: date.to_string(),
            idempotency_key: idempotency_key.to_string(),
            invoice_date: date.to_string(),
            lines: vec![CustomerInvoiceLine {
                description: "Hauliage delivery".to_string(),
                discount_percent: Some("0".to_string()),
                quantity: "1".to_string(),
                tax: Some(json!({ "code": "VAT20", "rate_percent": "20" })),
                unit_price: unit_price.to_string(),
            }],
            rounding_minor_units: Some(2),
            source: SourceReference {
                system: "hauliage".to_string(),
                resource_type: "delivery".to_string(),
                resource_id: format!("delivery-{idempotency_key}"),
            },
        }
    }
}
