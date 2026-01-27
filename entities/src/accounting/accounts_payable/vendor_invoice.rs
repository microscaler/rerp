//! Vendor Invoice entity
//!
//! Specialized invoice entity for accounts payable with AP-specific fields.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "vendor_invoices"]
#[skip_from_row]
#[table_comment = "Vendor invoices for accounts payable tracking"]
// NOTE: Only index columns that exist on THIS table, not on the parent 'invoices' table.
// Columns like invoice_number, due_date, status exist only on 'invoices',
// not on 'vendor_invoices'. Index those on the Invoice entity only.
#[index = "idx_vendor_invoices_vendor_id(vendor_id)"]
pub struct VendorInvoice {
    #[primary_key]
    pub id: uuid::Uuid,

    // Link to base invoice
    #[foreign_key = "invoices(id) ON DELETE CASCADE"]
    #[unique]
    pub invoice_id: uuid::Uuid,

    // Vendor reference
    #[foreign_key = "vendors(id) ON DELETE RESTRICT"]
    #[indexed]
    pub vendor_id: uuid::Uuid,

    // AP-specific fields
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub outstanding_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    pub days_until_due: i32, // Days until due_date (negative if overdue)

    #[column_type = "VARCHAR(50)"]
    pub aging_bucket: Option<String>, // CURRENT, 1-30, 31-60, 61-90, 90+

    // 3-way matching
    pub purchase_order_id: Option<uuid::Uuid>, // Reference to PO
    #[column_type = "VARCHAR(50)"]
    pub matching_status: Option<String>, // NOT_MATCHED, PARTIAL, MATCHED

    // Approval workflow
    #[column_type = "VARCHAR(50)"]
    pub approval_status: Option<String>, // PENDING, APPROVED, REJECTED

    pub approved_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<uuid::Uuid>,

    // Early payment discount
    pub early_payment_discount_percent: Option<rust_decimal::Decimal>,
    pub early_payment_discount_date: Option<chrono::NaiveDate>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
