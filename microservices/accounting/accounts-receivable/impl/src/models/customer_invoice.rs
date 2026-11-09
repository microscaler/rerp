//! AR overlay state for one posted customer document.
//!
//! The row is a rebuildable overlay: truth lives in the immutable posted
//! facts (`accounting_posted_documents` + journals). Aging and status are
//! derived at read time from the facts (F1b); this table only persists
//! settlement state that postings cannot know (write-offs, last payment,
//! credit-gate outcome) and anchors F2 payment allocations.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "customer_invoices"]
#[table_comment = "Rebuildable AR overlay for posted customer documents"]
#[composite_unique = "tenant_id, legal_entity_id, invoice_id"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_customer_invoices_scope(tenant_id, legal_entity_id)"]
#[index = "idx_customer_invoices_customer(customer_id)"]
#[index = "idx_customer_invoices_status(status)"]
pub struct CustomerInvoice {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    /// The posted customer document this overlay tracks (invoice or note).
    #[indexed]
    pub invoice_id: uuid::Uuid,

    #[indexed]
    pub customer_id: uuid::Uuid,

    /// Posted total of the document at posting time (decimal-string contract).
    #[column_type = "NUMERIC(19, 6)"]
    pub original_amount: rust_decimal::Decimal,

    /// Overlay cache of the derived outstanding balance; reads recompute
    /// from posted facts, never from this column alone.
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub outstanding_amount: rust_decimal::Decimal,

    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    pub due_date: chrono::NaiveDate,

    /// Payment terms derived from the document/due-date gap at projection
    /// time: `NET_15`, `NET_30`, `NET_45`, `NET_60`, `DUE_ON_RECEIPT` or CUSTOM.
    #[column_type = "VARCHAR(20)"]
    pub terms: String,

    /// AR settlement status (not the posted-document status): DRAFT,
    /// CONFIRMED, PARTIAL, PAID, `WRITTEN_OFF`, DISPUTED.
    #[default_value = "'CONFIRMED'"]
    #[column_type = "VARCHAR(20)"]
    pub status: String,

    /// Credit-limit gate outcome; null until customer credit data exists.
    pub credit_limit_check: Option<bool>,

    #[column_type = "NUMERIC(19, 6)"]
    pub write_off_amount: Option<rust_decimal::Decimal>,
    pub write_off_date: Option<chrono::NaiveDate>,

    #[column_type = "VARCHAR(1000)"]
    pub write_off_reason: Option<String>,

    pub last_payment_date: Option<chrono::NaiveDate>,

    #[column_type = "NUMERIC(19, 6)"]
    pub last_payment_amount: Option<rust_decimal::Decimal>,

    #[column_type = "JSONB"]
    pub metadata: Option<serde_json::Value>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    pub updated_at: chrono::NaiveDateTime,
}
