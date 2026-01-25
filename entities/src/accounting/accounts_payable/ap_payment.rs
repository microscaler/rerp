//! AP Payment entity
//!
//! Vendor payments applied to invoices.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "ap_payments"]
#[skip_from_row]
#[table_comment = "Vendor payments for accounts payable"]
#[index = "idx_ap_payments_vendor_id(vendor_id)"]
#[index = "idx_ap_payments_payment_date(payment_date)"]
#[index = "idx_ap_payments_payment_method(payment_method)"]
pub struct ApPayment {
    #[primary_key]
    pub id: uuid::Uuid,

    // Vendor reference
    #[foreign_key = "vendors(id) ON DELETE RESTRICT"]
    #[indexed]
    pub vendor_id: uuid::Uuid,

    // Payment details
    #[unique]
    #[column_type = "VARCHAR(100)"]
    pub payment_number: String,

    #[indexed]
    pub payment_date: chrono::NaiveDate,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub payment_amount: rust_decimal::Decimal,

    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Payment method
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub payment_method: String, // CHECK, WIRE, ACH, etc.

    #[column_type = "VARCHAR(100)"]
    pub payment_reference: Option<String>, // Check number, etc.

    // Bank account
    pub bank_account_id: Option<uuid::Uuid>, // Bank account payment was made from

    // Status
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, POSTED, RECONCILED, CANCELLED

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub applied_amount: rust_decimal::Decimal,

    // Multi-company
    pub company_id: Option<uuid::Uuid>,

    // Notes
    pub notes: Option<String>,

    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
