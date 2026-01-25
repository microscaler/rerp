//! Customer Invoice entity
//!
//! Specialized invoice entity for accounts receivable with AR-specific fields.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "customer_invoices"]
#[skip_from_row]
#[table_comment = "Customer invoices for accounts receivable tracking"]
#[index = "idx_customer_invoices_customer_id(customer_id)"]
pub struct CustomerInvoice {
    #[primary_key]
    pub id: uuid::Uuid,

    // Link to base invoice
    #[foreign_key = "invoices(id) ON DELETE CASCADE"]
    #[unique]
    pub invoice_id: uuid::Uuid,

    // Customer reference
    #[foreign_key = "customers(id) ON DELETE RESTRICT"]
    #[indexed]
    pub customer_id: uuid::Uuid,

    // AR-specific fields
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub outstanding_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    pub days_overdue: i32, // Calculated field: days since due_date

    #[column_type = "VARCHAR(50)"]
    pub aging_bucket: Option<String>, // CURRENT, 1-30, 31-60, 61-90, 90+

    // Credit limit tracking
    pub credit_limit: Option<rust_decimal::Decimal>,
    pub credit_used: Option<rust_decimal::Decimal>,

    // Collection workflow
    #[column_type = "VARCHAR(50)"]
    pub collection_status: Option<String>, // NORMAL, WARNING, COLLECTION, LEGAL

    pub last_payment_date: Option<chrono::NaiveDate>,
    pub last_payment_amount: Option<rust_decimal::Decimal>,

    // Write-off information
    pub write_off_amount: Option<rust_decimal::Decimal>,
    pub write_off_date: Option<chrono::NaiveDate>,
    pub write_off_reason: Option<String>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
