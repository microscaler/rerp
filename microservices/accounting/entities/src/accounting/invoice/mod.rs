//! Invoice service entities
//!
//! Entities for managing customer and vendor invoices.

// Preserve the public entity path used by the registry and generated migrations.
#[allow(clippy::module_inception)]
pub mod invoice;
pub mod invoice_line;

pub use invoice::Invoice;
pub use invoice_line::InvoiceLine;
