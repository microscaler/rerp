//! Invoice service entities
//!
//! Entities for managing customer and vendor invoices.

pub mod invoice;
pub mod invoice_line;

pub use invoice::Invoice;
pub use invoice_line::InvoiceLine;
