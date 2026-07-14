//! Typed persistence models for the first controlled invoice-to-ledger slice.
//!
//! These models intentionally coexist with the legacy schema-only accounting
//! inventory while the active runtime is narrowed. Every tenant-owned row
//! carries `tenant_id` directly so PostgreSQL RLS can fail closed without joins.

pub mod account;
pub mod audit_event;
pub mod fiscal_period;
pub mod idempotency_record;
pub mod journal_entry;
pub mod journal_line;
pub mod legal_entity;
pub mod posted_document;
pub mod posted_document_line;

pub use account::AccountingAccount;
pub use audit_event::AccountingAuditEvent;
pub use fiscal_period::AccountingFiscalPeriod;
pub use idempotency_record::AccountingIdempotencyRecord;
pub use journal_entry::AccountingJournalEntry;
pub use journal_line::AccountingJournalLine;
pub use legal_entity::AccountingLegalEntity;
pub use posted_document::AccountingPostedDocument;
pub use posted_document_line::AccountingPostedDocumentLine;
