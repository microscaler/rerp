use std::fmt;

use rust_decimal::Decimal;

/// Upper bound for a merged per-line selection total: NUMERIC(19,6) stores
/// at most 13 integer digits, so the merged quantity must stay below 1e13.
pub const MAX_QUANTITY_SUM: &str = "10000000000000 (NUMERIC(19,6) integer digits)";
/// journal-line descriptions are stored in VARCHAR(1000).
pub const MAX_NOTE_DESCRIPTION_LEN: usize = 1000;

/// A rejected accounting instruction or an internally invalid posting plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountingError {
    EmptyField(&'static str),
    InvalidCurrency(String),
    InvalidIdempotencyKey,
    InvalidPeriod,
    ClosedPeriod,
    DateOutsidePeriod,
    DueDateBeforeInvoiceDate,
    NoInvoiceLines,
    TooManyInvoiceLines,
    EmptyDescription(usize),
    NonPositiveQuantity(usize),
    NegativeUnitPrice(usize),
    InvalidDiscount(usize),
    InvalidTaxRate(usize),
    UnbalancedJournal,
    InvalidJournalLine(usize),
    TenantMismatch,
    LegalEntityMismatch,
    NotCustomerInvoice,
    OverCredit {
        line_number: u32,
        requested: Decimal,
        remaining: Decimal,
    },
    TotalQuantityTooLarge {
        line_number: u32,
    },
    DescriptionTooLong,
}

impl fmt::Display for AccountingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} must not be empty"),
            Self::InvalidCurrency(code) => write!(formatter, "invalid ISO currency code: {code}"),
            Self::InvalidIdempotencyKey => write!(
                formatter,
                "idempotency key must contain 1 to 200 characters"
            ),
            Self::InvalidPeriod => {
                write!(formatter, "fiscal period start must not be after its end")
            }
            Self::ClosedPeriod => write!(formatter, "fiscal period is closed"),
            Self::DateOutsidePeriod => {
                write!(formatter, "invoice date is outside the fiscal period")
            }
            Self::DueDateBeforeInvoiceDate => {
                write!(formatter, "due date must not precede invoice date")
            }
            Self::NoInvoiceLines => write!(formatter, "an invoice requires at least one line"),
            Self::TooManyInvoiceLines => {
                write!(formatter, "an invoice may contain at most 1,000 lines")
            }
            Self::EmptyDescription(line) => {
                write!(formatter, "line {line} description must not be empty")
            }
            Self::NonPositiveQuantity(line) => {
                write!(formatter, "line {line} quantity must be positive")
            }
            Self::NegativeUnitPrice(line) => {
                write!(formatter, "line {line} unit price must not be negative")
            }
            Self::InvalidDiscount(line) => write!(
                formatter,
                "line {line} discount must be between 0 and 100 percent"
            ),
            Self::InvalidTaxRate(line) => write!(
                formatter,
                "line {line} tax rate must be between 0 and 100 percent"
            ),
            Self::UnbalancedJournal => write!(formatter, "journal debit and credit totals differ"),
            Self::InvalidJournalLine(line) => write!(
                formatter,
                "journal line {line} must contain exactly one positive debit or credit"
            ),
            Self::TenantMismatch => write!(
                formatter,
                "accounting documents belong to different tenants"
            ),
            Self::LegalEntityMismatch => write!(
                formatter,
                "accounting documents belong to different legal entities"
            ),
            Self::NotCustomerInvoice => {
                write!(formatter, "only a posted customer invoice can be credited or debited")
            }
            Self::OverCredit {
                line_number,
                requested,
                remaining,
            } => write!(
                formatter,
                "line {line_number}: requested credit quantity {requested} exceeds the remaining creditable quantity {remaining}"
            ),
            Self::TotalQuantityTooLarge { line_number } => write!(
                formatter,
                "line {line_number}: merged quantity exceeds the {MAX_QUANTITY_SUM} storage limit"
            ),
            Self::DescriptionTooLong => write!(
                formatter,
                "the derived document description exceeds the 1000-character journal limit"
            ),
        }
    }
}

impl std::error::Error for AccountingError {}
