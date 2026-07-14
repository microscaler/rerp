use crate::AccountingError;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

/// An uppercase, three-letter ISO 4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyCode(String);

impl CurrencyCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = AccountingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 3 && value.bytes().all(|byte| byte.is_ascii_uppercase()) {
            Ok(Self(value.to_owned()))
        } else {
            Err(AccountingError::InvalidCurrency(value.to_owned()))
        }
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = AccountingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<CurrencyCode> for String {
    fn from(value: CurrencyCode) -> Self {
        value.0
    }
}

/// The caller-selected currency precision used for all line calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundingPolicy {
    minor_units: u32,
}

impl RoundingPolicy {
    pub fn new(minor_units: u32) -> Result<Self, AccountingError> {
        if minor_units <= 6 {
            Ok(Self { minor_units })
        } else {
            Err(AccountingError::InvalidCurrency(format!(
                "unsupported currency precision {minor_units}"
            )))
        }
    }

    pub fn minor_units(self) -> u32 {
        self.minor_units
    }

    pub fn round(self, amount: Decimal) -> Decimal {
        amount.round_dp_with_strategy(self.minor_units, RoundingStrategy::MidpointAwayFromZero)
    }
}

/// Lifecycle state of a fiscal period relevant to posting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiscalPeriodState {
    Open,
    Closed,
}

/// Authenticated execution facts. The HTTP payload must not supply these.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostingContext {
    pub tenant_id: String,
    pub legal_entity_id: Uuid,
    pub subject_id: Uuid,
    pub fiscal_period_id: Uuid,
    pub fiscal_period_start: NaiveDate,
    pub fiscal_period_end: NaiveDate,
    pub fiscal_period_state: FiscalPeriodState,
    pub posted_at: NaiveDateTime,
}

impl PostingContext {
    pub(crate) fn validate(&self, document_date: NaiveDate) -> Result<(), AccountingError> {
        if self.tenant_id.trim().is_empty() {
            return Err(AccountingError::EmptyField("tenant_id"));
        }
        if self.fiscal_period_start > self.fiscal_period_end {
            return Err(AccountingError::InvalidPeriod);
        }
        if self.fiscal_period_state == FiscalPeriodState::Closed {
            return Err(AccountingError::ClosedPeriod);
        }
        if document_date < self.fiscal_period_start || document_date > self.fiscal_period_end {
            return Err(AccountingError::DateOutsidePeriod);
        }
        Ok(())
    }
}

/// A stable reference to the commercial fact in the source product.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceReference {
    pub system: String,
    pub resource_type: String,
    pub resource_id: String,
}

impl SourceReference {
    pub(crate) fn validate(&self) -> Result<(), AccountingError> {
        for (name, value) in [
            ("source.system", &self.system),
            ("source.resource_type", &self.resource_type),
            ("source.resource_id", &self.resource_id),
        ] {
            if value.trim().is_empty() {
                return Err(AccountingError::EmptyField(name));
            }
        }
        Ok(())
    }
}

/// Consumer retry key. Persistence enforces uniqueness within an authenticated tenant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for IdempotencyKey {
    type Error = AccountingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let length = value.chars().count();
        if (1..=200).contains(&length) && value.trim().len() == value.len() {
            Ok(Self(value.to_owned()))
        } else {
            Err(AccountingError::InvalidIdempotencyKey)
        }
    }
}

impl TryFrom<String> for IdempotencyKey {
    type Error = AccountingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<IdempotencyKey> for String {
    fn from(value: IdempotencyKey) -> Self {
        value.0
    }
}

/// IDs and legal document number allocated by the transactional persistence layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostingIdentifiers {
    pub invoice_id: Uuid,
    pub journal_entry_id: Uuid,
    pub invoice_number: String,
    pub journal_entry_number: String,
}

impl PostingIdentifiers {
    pub(crate) fn validate(&self) -> Result<(), AccountingError> {
        if self.invoice_number.trim().is_empty() {
            return Err(AccountingError::EmptyField("invoice_number"));
        }
        if self.journal_entry_number.trim().is_empty() {
            return Err(AccountingError::EmptyField("journal_entry_number"));
        }
        Ok(())
    }
}
