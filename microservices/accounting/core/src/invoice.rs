use crate::error::MAX_NOTE_DESCRIPTION_LEN;
use crate::types::{
    CurrencyCode, IdempotencyKey, PostingContext, PostingIdentifiers, RoundingPolicy,
    SourceReference,
};
use crate::AccountingError;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const ONE_HUNDRED: Decimal = Decimal::from_parts(100, 0, 0, false, 0);
const MAX_INVOICE_LINES: usize = 1_000;

/// An optional line-level tax snapshot and its GL destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxInput {
    pub code: String,
    pub rate_percent: Decimal,
    pub liability_account_id: Uuid,
}

/// A commercial line plus the revenue/tax account mapping selected by RERP policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceLineInput {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_percent: Decimal,
    pub revenue_account_id: Uuid,
    pub tax: Option<TaxInput>,
}

/// Domain-neutral instruction for issuing and posting a customer invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerInvoiceInstruction {
    pub idempotency_key: IdempotencyKey,
    pub source: SourceReference,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub currency: CurrencyCode,
    pub rounding: RoundingPolicy,
    pub receivable_account_id: Uuid,
    pub lines: Vec<InvoiceLineInput>,
}

/// Persisted line values. They are snapshots, not recalculated product references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceLineSnapshot {
    pub line_number: u32,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_percent: Decimal,
    pub gross_amount: Decimal,
    pub discount_amount: Decimal,
    pub net_amount: Decimal,
    pub tax_code: Option<String>,
    pub tax_rate_percent: Option<Decimal>,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub revenue_account_id: Uuid,
    pub tax_liability_account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountingDocumentType {
    CustomerInvoice,
    CustomerCreditNote,
    CustomerDebitNote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountingDocumentStatus {
    Posted,
}

/// Immutable accounting representation of an issued invoice or credit note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceSnapshot {
    pub id: Uuid,
    pub tenant_id: String,
    pub legal_entity_id: Uuid,
    pub fiscal_period_id: Uuid,
    pub document_number: String,
    pub document_type: AccountingDocumentType,
    pub status: AccountingDocumentStatus,
    pub original_document_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub source: SourceReference,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub currency: CurrencyCode,
    pub posted_at: NaiveDateTime,
    pub subtotal: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub lines: Vec<InvoiceLineSnapshot>,
}

/// One side of a journal line; represented as an enum so both sides cannot be set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalSide {
    Debit,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalLine {
    pub line_number: u32,
    pub account_id: Uuid,
    pub description: String,
    pub side: JournalSide,
    pub amount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub tenant_id: String,
    pub legal_entity_id: Uuid,
    pub fiscal_period_id: Uuid,
    pub entry_number: String,
    pub entry_date: NaiveDate,
    pub source_document_id: Uuid,
    pub currency: CurrencyCode,
    pub lines: Vec<JournalLine>,
    pub posted_at: NaiveDateTime,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
}

impl JournalEntry {
    pub fn validate(&self) -> Result<(), AccountingError> {
        let mut total_debit = Decimal::ZERO;
        let mut total_credit = Decimal::ZERO;
        for (index, line) in self.lines.iter().enumerate() {
            if line.amount <= Decimal::ZERO {
                return Err(AccountingError::InvalidJournalLine(index + 1));
            }
            match line.side {
                JournalSide::Debit => total_debit += line.amount,
                JournalSide::Credit => total_credit += line.amount,
            }
        }
        if total_debit != total_credit
            || total_debit != self.total_debit
            || total_credit != self.total_credit
        {
            return Err(AccountingError::UnbalancedJournal);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditAction {
    InvoicePosted,
    CreditNotePosted,
    DebitNotePosted,
}

/// Audit fact prepared with the posting plan and persisted in the same transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub tenant_id: String,
    pub legal_entity_id: Uuid,
    pub subject_id: Uuid,
    pub action: AuditAction,
    pub document_id: Uuid,
    pub original_document_id: Option<Uuid>,
    pub occurred_at: NaiveDateTime,
}

/// The complete atomic result. Persistence must write all members or none of them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostingPlan {
    pub idempotency_key: IdempotencyKey,
    pub request_fingerprint: String,
    pub invoice: InvoiceSnapshot,
    pub journal: JournalEntry,
    pub audit_event: AuditEvent,
}

/// Validate, calculate and produce a balanced customer-invoice posting plan.
pub fn post_customer_invoice(
    context: &PostingContext,
    identifiers: &PostingIdentifiers,
    instruction: &CustomerInvoiceInstruction,
) -> Result<PostingPlan, AccountingError> {
    context.validate(instruction.invoice_date)?;
    identifiers.validate()?;
    instruction.source.validate()?;
    validate_instruction(instruction)?;

    let mut subtotal = Decimal::ZERO;
    let mut discount_amount = Decimal::ZERO;
    let mut tax_amount = Decimal::ZERO;
    let mut total_amount = Decimal::ZERO;
    let mut invoice_lines = Vec::with_capacity(instruction.lines.len());

    for (index, input) in instruction.lines.iter().enumerate() {
        let line = calculate_line(index + 1, input, instruction.rounding)?;
        subtotal += line.gross_amount;
        discount_amount += line.discount_amount;
        tax_amount += line.tax_amount;
        total_amount += line.total_amount;
        invoice_lines.push(line);
    }

    subtotal = instruction.rounding.round(subtotal);
    discount_amount = instruction.rounding.round(discount_amount);
    tax_amount = instruction.rounding.round(tax_amount);
    total_amount = instruction.rounding.round(total_amount);

    let invoice = InvoiceSnapshot {
        id: identifiers.invoice_id,
        tenant_id: context.tenant_id.clone(),
        legal_entity_id: context.legal_entity_id,
        fiscal_period_id: context.fiscal_period_id,
        document_number: identifiers.invoice_number.clone(),
        document_type: AccountingDocumentType::CustomerInvoice,
        status: AccountingDocumentStatus::Posted,
        original_document_id: None,
        customer_id: instruction.customer_id,
        source: instruction.source.clone(),
        invoice_date: instruction.invoice_date,
        due_date: instruction.due_date,
        currency: instruction.currency.clone(),
        posted_at: context.posted_at,
        subtotal,
        discount_amount,
        tax_amount,
        total_amount,
        lines: invoice_lines,
    };

    let journal = customer_invoice_journal(context, identifiers, instruction, &invoice);
    journal.validate()?;

    Ok(PostingPlan {
        idempotency_key: instruction.idempotency_key.clone(),
        request_fingerprint: fingerprint(context, instruction),
        audit_event: AuditEvent {
            tenant_id: context.tenant_id.clone(),
            legal_entity_id: context.legal_entity_id,
            subject_id: context.subject_id,
            action: AuditAction::InvoicePosted,
            document_id: invoice.id,
            original_document_id: None,
            occurred_at: context.posted_at,
        },
        invoice,
        journal,
    })
}

/// Create a full credit note and exactly reverse the original accounting entry.
pub fn credit_customer_invoice(
    context: &PostingContext,
    identifiers: &PostingIdentifiers,
    idempotency_key: IdempotencyKey,
    source: SourceReference,
    original: &PostingPlan,
    reason: &str,
) -> Result<PostingPlan, AccountingError> {
    let credit_date = context.posted_at.date();
    context.validate(credit_date)?;
    identifiers.validate()?;
    source.validate()?;
    if reason.trim().is_empty() {
        return Err(AccountingError::EmptyField("credit_reason"));
    }
    let credit_note_description =
        note_description("Credit", &original.invoice.document_number, reason)?;
    if original.invoice.document_type != AccountingDocumentType::CustomerInvoice {
        return Err(AccountingError::NotCustomerInvoice);
    }
    if original.invoice.tenant_id != context.tenant_id {
        return Err(AccountingError::TenantMismatch);
    }
    if original.invoice.legal_entity_id != context.legal_entity_id {
        return Err(AccountingError::LegalEntityMismatch);
    }

    let invoice = InvoiceSnapshot {
        id: identifiers.invoice_id,
        tenant_id: context.tenant_id.clone(),
        legal_entity_id: context.legal_entity_id,
        fiscal_period_id: context.fiscal_period_id,
        document_number: identifiers.invoice_number.clone(),
        document_type: AccountingDocumentType::CustomerCreditNote,
        status: AccountingDocumentStatus::Posted,
        original_document_id: Some(original.invoice.id),
        customer_id: original.invoice.customer_id,
        source,
        invoice_date: credit_date,
        due_date: credit_date,
        currency: original.invoice.currency.clone(),
        posted_at: context.posted_at,
        subtotal: original.invoice.subtotal,
        discount_amount: original.invoice.discount_amount,
        tax_amount: original.invoice.tax_amount,
        total_amount: original.invoice.total_amount,
        lines: original.invoice.lines.clone(),
    };

    let lines = original
        .journal
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| JournalLine {
            line_number: (index + 1) as u32,
            account_id: line.account_id,
            description: credit_note_description.clone(),
            side: match line.side {
                JournalSide::Debit => JournalSide::Credit,
                JournalSide::Credit => JournalSide::Debit,
            },
            amount: line.amount,
        })
        .collect();
    let journal = JournalEntry {
        id: identifiers.journal_entry_id,
        tenant_id: context.tenant_id.clone(),
        legal_entity_id: context.legal_entity_id,
        fiscal_period_id: context.fiscal_period_id,
        entry_number: identifiers.journal_entry_number.clone(),
        entry_date: invoice.invoice_date,
        source_document_id: invoice.id,
        currency: invoice.currency.clone(),
        lines,
        posted_at: context.posted_at,
        total_debit: original.journal.total_credit,
        total_credit: original.journal.total_debit,
    };
    journal.validate()?;

    let request_fingerprint = credit_fingerprint(context, &idempotency_key, &invoice, reason);
    Ok(PostingPlan {
        idempotency_key,
        request_fingerprint,
        audit_event: AuditEvent {
            tenant_id: context.tenant_id.clone(),
            legal_entity_id: context.legal_entity_id,
            subject_id: context.subject_id,
            action: AuditAction::CreditNotePosted,
            document_id: invoice.id,
            original_document_id: Some(original.invoice.id),
            occurred_at: context.posted_at,
        },
        invoice,
        journal,
    })
}

/// One explicit line selection against an original line.
///
/// Shared by partial credit notes and customer debit notes: both reference
/// an original line by number and inherit its unit price, discount and tax
/// facts, differing only in how the quantity is validated and which side the
/// journal entry lands on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditLineSelection {
    pub original_line_number: u32,
    pub quantity: Decimal,
}

/// Create a PARTIAL credit note for explicitly selected original lines.
///
/// The original document is never modified: the credit note is its own
/// posted document linked via `original_document_id`. Quantities are
/// validated against `credited_quantities` (what previous credit notes —
/// full or partial — already consumed per original line), so repeated
/// partial credits accumulate deterministically and an over-credit is
/// rejected with [`AccountingError::OverCredit`]. Selected original line
/// numbers are preserved on the credit note lines so the creditable
/// balance remains computable.
#[allow(clippy::too_many_arguments)]
pub fn partial_credit_customer_invoice(
    context: &PostingContext,
    identifiers: &PostingIdentifiers,
    idempotency_key: IdempotencyKey,
    source: SourceReference,
    receivable_account_id: Uuid,
    rounding: RoundingPolicy,
    credited_quantities: &std::collections::BTreeMap<u32, Decimal>,
    original: &PostingPlan,
    reason: &str,
    selections: &[CreditLineSelection],
) -> Result<PostingPlan, AccountingError> {
    let credit_date = context.posted_at.date();
    context.validate(credit_date)?;
    identifiers.validate()?;
    source.validate()?;
    if reason.trim().is_empty() {
        return Err(AccountingError::EmptyField("credit_reason"));
    }
    if original.invoice.document_type != AccountingDocumentType::CustomerInvoice {
        return Err(AccountingError::NotCustomerInvoice);
    }
    if original.invoice.tenant_id != context.tenant_id {
        return Err(AccountingError::TenantMismatch);
    }
    if original.invoice.legal_entity_id != context.legal_entity_id {
        return Err(AccountingError::LegalEntityMismatch);
    }
    if selections.is_empty() {
        return Err(AccountingError::NoInvoiceLines);
    }

    // Merge duplicate selections per original line before validation so
    // repeated rows for one line can never smuggle past the remainder.
    let requested = merged_selections(selections)?;

    let mut lines = Vec::with_capacity(requested.len());
    for (line_number, quantity) in &requested {
        let original_line = original
            .invoice
            .lines
            .iter()
            .find(|line| line.line_number == *line_number)
            .ok_or(AccountingError::InvalidJournalLine(*line_number as usize))?;
        let remaining = original_line.quantity
            - credited_quantities
                .get(line_number)
                .copied()
                .unwrap_or_default();
        if *quantity > remaining {
            return Err(AccountingError::OverCredit {
                line_number: *line_number,
                requested: *quantity,
                remaining,
            });
        }
        lines.push(line_input_from_original(original_line, *quantity));
    }

    let instruction = CustomerInvoiceInstruction {
        idempotency_key: idempotency_key.clone(),
        source,
        customer_id: original.invoice.customer_id,
        invoice_date: credit_date,
        due_date: credit_date,
        currency: original.invoice.currency.clone(),
        rounding,
        receivable_account_id,
        lines,
    };
    let mut plan = post_customer_invoice(context, identifiers, &instruction)?;

    // Re-tag the computed plan as a credit note: same arithmetic, sides
    // reversed, original linkage added, selection line numbers preserved.
    plan.invoice.document_type = AccountingDocumentType::CustomerCreditNote;
    plan.invoice.original_document_id = Some(original.invoice.id);
    for (line, line_number) in plan.invoice.lines.iter_mut().zip(requested.keys().copied()) {
        line.line_number = line_number;
    }
    let credit_note_description =
        note_description("Credit", &original.invoice.document_number, reason)?;
    for line in &mut plan.journal.lines {
        line.side = match line.side {
            JournalSide::Debit => JournalSide::Credit,
            JournalSide::Credit => JournalSide::Debit,
        };
        if line.account_id == instruction.receivable_account_id {
            line.description = credit_note_description.clone();
        }
    }
    let debit = plan.journal.total_credit;
    let credit = plan.journal.total_debit;
    plan.journal.total_debit = debit;
    plan.journal.total_credit = credit;
    plan.journal.validate()?;

    plan.request_fingerprint =
        partial_credit_fingerprint(context, &idempotency_key, &plan.invoice, reason);
    plan.audit_event.action = AuditAction::CreditNotePosted;
    plan.audit_event.original_document_id = Some(original.invoice.id);
    Ok(plan)
}

/// Create a customer DEBIT NOTE charging additional quantities against an
/// original customer invoice.
///
/// A debit note increases what the customer owes: the journal keeps the
/// invoice direction (receivable debited, revenue and tax credited) and is
/// priced from the selected original lines — unit price, discount and tax
/// facts are inherited exactly like a partial credit. Unlike a credit there
/// is NO quantity cap: additional charges are not bounded by the original
/// invoice quantities. The original document is never modified; the debit
/// note is its own posted document linked via `original_document_id`, and
/// selected original line numbers are preserved on the debit lines.
/// A selection must round to a positive charge at the invoice precision:
/// a positive quantity priced to zero is rejected as an invalid journal
/// line, exactly as for an invoice line.
#[allow(clippy::too_many_arguments)]
pub fn debit_customer_invoice(
    context: &PostingContext,
    identifiers: &PostingIdentifiers,
    idempotency_key: IdempotencyKey,
    source: SourceReference,
    receivable_account_id: Uuid,
    rounding: RoundingPolicy,
    original: &PostingPlan,
    reason: &str,
    selections: &[CreditLineSelection],
) -> Result<PostingPlan, AccountingError> {
    let debit_date = context.posted_at.date();
    context.validate(debit_date)?;
    identifiers.validate()?;
    source.validate()?;
    if reason.trim().is_empty() {
        return Err(AccountingError::EmptyField("debit_reason"));
    }
    let debit_note_description =
        note_description("Debit", &original.invoice.document_number, reason)?;
    if original.invoice.document_type != AccountingDocumentType::CustomerInvoice {
        return Err(AccountingError::NotCustomerInvoice);
    }
    if original.invoice.tenant_id != context.tenant_id {
        return Err(AccountingError::TenantMismatch);
    }
    if original.invoice.legal_entity_id != context.legal_entity_id {
        return Err(AccountingError::LegalEntityMismatch);
    }
    if selections.is_empty() {
        return Err(AccountingError::NoInvoiceLines);
    }

    // Duplicate rows merge like credits do, so a repeated line can never
    // smuggle a different quantity past the fingerprint.
    let requested = merged_selections(selections)?;

    let mut lines = Vec::with_capacity(requested.len());
    for (line_number, quantity) in &requested {
        let original_line = original
            .invoice
            .lines
            .iter()
            .find(|line| line.line_number == *line_number)
            .ok_or(AccountingError::InvalidJournalLine(*line_number as usize))?;
        lines.push(line_input_from_original(original_line, *quantity));
    }

    let instruction = CustomerInvoiceInstruction {
        idempotency_key: idempotency_key.clone(),
        source,
        customer_id: original.invoice.customer_id,
        invoice_date: debit_date,
        due_date: debit_date,
        currency: original.invoice.currency.clone(),
        rounding,
        receivable_account_id,
        lines,
    };
    let mut plan = post_customer_invoice(context, identifiers, &instruction)?;

    // Same arithmetic as an invoice, invoice-direction journal retained
    // as-is: a debit note ADDS to the receivable. Only the identity,
    // linkage, audit action and fingerprint differ.
    plan.invoice.document_type = AccountingDocumentType::CustomerDebitNote;
    plan.invoice.original_document_id = Some(original.invoice.id);
    for (line, line_number) in plan.invoice.lines.iter_mut().zip(requested.keys().copied()) {
        line.line_number = line_number;
    }
    for line in &mut plan.journal.lines {
        if line.account_id == instruction.receivable_account_id {
            line.description = debit_note_description.clone();
        }
    }
    plan.request_fingerprint = debit_fingerprint(context, &idempotency_key, &plan.invoice, reason);
    plan.audit_event.action = AuditAction::DebitNotePosted;
    plan.audit_event.original_document_id = Some(original.invoice.id);
    Ok(plan)
}

/// Merge duplicate per-line selections, rejecting non-positive quantities.
/// Merged totals must also stay inside NUMERIC(19,6) storage (13 integer
/// digits): duplicate rows that each fit the per-row pattern could
/// otherwise sum past the column limit and fail at the database.
fn merged_selections(
    selections: &[CreditLineSelection],
) -> Result<std::collections::BTreeMap<u32, Decimal>, AccountingError> {
    let mut requested: std::collections::BTreeMap<u32, Decimal> = Default::default();
    for selection in selections {
        if selection.quantity <= Decimal::ZERO {
            return Err(AccountingError::NonPositiveQuantity(
                selection.original_line_number as usize,
            ));
        }
        *requested
            .entry(selection.original_line_number)
            .or_insert(Decimal::ZERO) += selection.quantity;
    }
    let storage_bound = Decimal::from(10_000_000_000_000i64); // NUMERIC(19,6) integer-digit limit
    for (line_number, quantity) in &requested {
        if quantity >= &storage_bound {
            return Err(AccountingError::TotalQuantityTooLarge {
                line_number: *line_number,
            });
        }
    }
    Ok(requested)
}

/// Compose a derived-document journal description and prove it fits the
/// VARCHAR(1000) journal-line column BEFORE posting, so an over-long
/// reason fails as a request error instead of a mid-transaction insert
/// failure. PostgreSQL varchar(n) counts characters, not bytes.
fn note_description(
    prefix: &str,
    document_number: &str,
    reason: &str,
) -> Result<String, AccountingError> {
    let description = format!("{prefix} {document_number}: {reason}");
    if description.chars().count() > MAX_NOTE_DESCRIPTION_LEN {
        return Err(AccountingError::DescriptionTooLong);
    }
    Ok(description)
}

/// Price a selection from its original line: unit price, discount and tax
/// facts are inherited so derived documents always bill at contract rates.
fn line_input_from_original(
    original_line: &InvoiceLineSnapshot,
    quantity: Decimal,
) -> InvoiceLineInput {
    let tax = match (
        original_line.tax_code.clone(),
        original_line.tax_rate_percent,
        original_line.tax_liability_account_id,
    ) {
        (Some(code), Some(rate_percent), Some(liability_account_id)) => Some(TaxInput {
            code,
            rate_percent,
            liability_account_id,
        }),
        _ => None,
    };
    InvoiceLineInput {
        description: original_line.description.clone(),
        quantity,
        unit_price: original_line.unit_price,
        discount_percent: original_line.discount_percent,
        revenue_account_id: original_line.revenue_account_id,
        tax,
    }
}

fn partial_credit_fingerprint(
    context: &PostingContext,
    idempotency_key: &IdempotencyKey,
    invoice: &InvoiceSnapshot,
    reason: &str,
) -> String {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, context.tenant_id.as_bytes());
    hash_part(&mut hasher, context.legal_entity_id.as_bytes());
    hash_part(&mut hasher, idempotency_key.as_str().as_bytes());
    hash_part(
        &mut hasher,
        invoice.original_document_id.unwrap_or_default().as_bytes(),
    );
    hash_part(&mut hasher, reason.as_bytes());
    // The source reference is part of the request: a replayed key with a
    // different source must surface as an idempotency conflict, not a 200.
    hash_part(&mut hasher, invoice.source.system.as_bytes());
    hash_part(&mut hasher, invoice.source.resource_type.as_bytes());
    hash_part(&mut hasher, invoice.source.resource_id.as_bytes());
    // Unlike a full reversal, the selected quantities ARE the request:
    // bind them into the fingerprint so a replayed key with a different
    // selection can never be accepted as identical.
    for line in &invoice.lines {
        let part = format!(
            "{}|{}|{}",
            line.line_number,
            line.quantity.normalize(),
            line.total_amount.normalize()
        );
        hash_part(&mut hasher, part.as_bytes());
    }
    let digest = hasher.finalize();
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn debit_fingerprint(
    context: &PostingContext,
    idempotency_key: &IdempotencyKey,
    invoice: &InvoiceSnapshot,
    reason: &str,
) -> String {
    let mut hasher = Sha256::new();
    // Document-type discriminator FIRST: a debit note replayed against the
    // same key/original/selection as a credit note must never fingerprint
    // identically, or the idempotency layer would replay the wrong document.
    hash_part(&mut hasher, b"CUSTOMER_DEBIT_NOTE");
    hash_part(&mut hasher, context.tenant_id.as_bytes());
    hash_part(&mut hasher, context.legal_entity_id.as_bytes());
    hash_part(&mut hasher, idempotency_key.as_str().as_bytes());
    hash_part(
        &mut hasher,
        invoice.original_document_id.unwrap_or_default().as_bytes(),
    );
    hash_part(&mut hasher, reason.as_bytes());
    hash_part(&mut hasher, invoice.source.system.as_bytes());
    hash_part(&mut hasher, invoice.source.resource_type.as_bytes());
    hash_part(&mut hasher, invoice.source.resource_id.as_bytes());
    for line in &invoice.lines {
        let part = format!(
            "{}|{}|{}",
            line.line_number,
            line.quantity.normalize(),
            line.total_amount.normalize()
        );
        hash_part(&mut hasher, part.as_bytes());
    }
    let digest = hasher.finalize();
    hex_digest(&digest)
}

fn validate_instruction(instruction: &CustomerInvoiceInstruction) -> Result<(), AccountingError> {
    if instruction.due_date < instruction.invoice_date {
        return Err(AccountingError::DueDateBeforeInvoiceDate);
    }
    if instruction.lines.is_empty() {
        return Err(AccountingError::NoInvoiceLines);
    }
    if instruction.lines.len() > MAX_INVOICE_LINES {
        return Err(AccountingError::TooManyInvoiceLines);
    }
    Ok(())
}

fn calculate_line(
    line_number: usize,
    input: &InvoiceLineInput,
    rounding: RoundingPolicy,
) -> Result<InvoiceLineSnapshot, AccountingError> {
    if input.description.trim().is_empty() {
        return Err(AccountingError::EmptyDescription(line_number));
    }
    if input.quantity <= Decimal::ZERO {
        return Err(AccountingError::NonPositiveQuantity(line_number));
    }
    if input.unit_price < Decimal::ZERO {
        return Err(AccountingError::NegativeUnitPrice(line_number));
    }
    if input.discount_percent < Decimal::ZERO || input.discount_percent > ONE_HUNDRED {
        return Err(AccountingError::InvalidDiscount(line_number));
    }
    if let Some(tax) = &input.tax {
        if tax.code.trim().is_empty() {
            return Err(AccountingError::EmptyField("tax.code"));
        }
        if tax.rate_percent < Decimal::ZERO || tax.rate_percent > ONE_HUNDRED {
            return Err(AccountingError::InvalidTaxRate(line_number));
        }
    }

    let gross_amount = rounding.round(input.quantity * input.unit_price);
    let discount_amount = rounding.round(gross_amount * input.discount_percent / ONE_HUNDRED);
    let net_amount = rounding.round(gross_amount - discount_amount);
    let tax_amount = input.tax.as_ref().map_or(Decimal::ZERO, |tax| {
        rounding.round(net_amount * tax.rate_percent / ONE_HUNDRED)
    });
    let total_amount = rounding.round(net_amount + tax_amount);

    Ok(InvoiceLineSnapshot {
        line_number: line_number as u32,
        description: input.description.clone(),
        quantity: input.quantity,
        unit_price: input.unit_price,
        discount_percent: input.discount_percent,
        gross_amount,
        discount_amount,
        net_amount,
        tax_code: input.tax.as_ref().map(|tax| tax.code.clone()),
        tax_rate_percent: input.tax.as_ref().map(|tax| tax.rate_percent),
        tax_amount,
        total_amount,
        revenue_account_id: input.revenue_account_id,
        tax_liability_account_id: input.tax.as_ref().map(|tax| tax.liability_account_id),
    })
}

fn customer_invoice_journal(
    context: &PostingContext,
    identifiers: &PostingIdentifiers,
    instruction: &CustomerInvoiceInstruction,
    invoice: &InvoiceSnapshot,
) -> JournalEntry {
    let mut lines = Vec::with_capacity(1 + invoice.lines.len() * 2);
    lines.push(JournalLine {
        line_number: 1,
        account_id: instruction.receivable_account_id,
        description: format!("Receivable for {}", invoice.document_number),
        side: JournalSide::Debit,
        amount: invoice.total_amount,
    });

    for invoice_line in &invoice.lines {
        lines.push(JournalLine {
            line_number: (lines.len() + 1) as u32,
            account_id: invoice_line.revenue_account_id,
            description: invoice_line.description.clone(),
            side: JournalSide::Credit,
            amount: invoice_line.net_amount,
        });
        if invoice_line.tax_amount > Decimal::ZERO {
            if let Some(account_id) = invoice_line.tax_liability_account_id {
                lines.push(JournalLine {
                    line_number: (lines.len() + 1) as u32,
                    account_id,
                    description: format!("Tax: {}", invoice_line.description),
                    side: JournalSide::Credit,
                    amount: invoice_line.tax_amount,
                });
            }
        }
    }

    JournalEntry {
        id: identifiers.journal_entry_id,
        tenant_id: context.tenant_id.clone(),
        legal_entity_id: context.legal_entity_id,
        fiscal_period_id: context.fiscal_period_id,
        entry_number: identifiers.journal_entry_number.clone(),
        entry_date: invoice.invoice_date,
        source_document_id: invoice.id,
        currency: invoice.currency.clone(),
        lines,
        posted_at: context.posted_at,
        total_debit: invoice.total_amount,
        total_credit: invoice.total_amount,
    }
}

fn fingerprint(context: &PostingContext, instruction: &CustomerInvoiceInstruction) -> String {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, context.tenant_id.as_bytes());
    hash_part(&mut hasher, context.legal_entity_id.as_bytes());
    let encoded = serde_json::to_vec(instruction).unwrap_or_default();
    hash_part(&mut hasher, &encoded);
    let digest = hasher.finalize();
    hex_digest(&digest)
}

fn credit_fingerprint(
    context: &PostingContext,
    idempotency_key: &IdempotencyKey,
    invoice: &InvoiceSnapshot,
    reason: &str,
) -> String {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, context.tenant_id.as_bytes());
    hash_part(&mut hasher, context.legal_entity_id.as_bytes());
    hash_part(&mut hasher, idempotency_key.as_str().as_bytes());
    hash_part(
        &mut hasher,
        invoice.original_document_id.unwrap_or_default().as_bytes(),
    );
    hash_part(&mut hasher, reason.as_bytes());
    let digest = hasher.finalize();
    hex_digest(&digest)
}

fn hash_part(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
