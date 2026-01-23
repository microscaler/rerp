//! Accounting service entities
//!
//! This module contains all accounting-related entities organized by service domain.
//!
//! ## Services
//!
//! - `general_ledger` - Core accounting entities (Chart of Accounts, Journal Entries)
//! - `invoice` - Invoice management (Customer and Vendor Invoices)
//! - `accounts_receivable` - AR management (Customer Invoices, Payments, Aging)
//! - `accounts_payable` - AP management (Vendor Invoices, Payments, Aging)
//! - `asset` - Asset management (Fixed Assets, Depreciation)
//! - `bank_sync` - Bank synchronization (Bank Accounts, Transactions, Reconciliations)
//! - `budget` - Budget planning and tracking (Budgets, Periods, Actuals)
//! - `edi` - EDI processing (Documents, Transactions, Mappings)
//! - `financial_reports` - Financial reporting (Reports, Templates, Schedules)

pub mod general_ledger;
pub mod invoice;
pub mod accounts_receivable;
pub mod accounts_payable;
pub mod asset;
pub mod bank_sync;
pub mod budget;
pub mod edi;
pub mod financial_reports;

// Re-export entities for convenience
pub use general_ledger::*;
pub use invoice::*;
pub use accounts_receivable::*;
pub use accounts_payable::*;
pub use asset::*;
pub use bank_sync::*;
pub use budget::*;
pub use edi::*;
pub use financial_reports::*;