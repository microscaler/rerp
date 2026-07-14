//! AR Payment entity
//!
//! Customer payments applied to invoices.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "ar_payments"]
#[skip_from_row]
#[table_comment = "Customer payments for accounts receivable"]
#[index = "idx_ar_payments_customer_id(customer_id)"]
#[index = "idx_ar_payments_payment_date(payment_date)"]
#[index = "idx_ar_payments_payment_method(payment_method)"]
#[index = "idx_ar_payments_status(status)"]
pub struct ArPayment {
    #[primary_key]
    pub id: uuid::Uuid,

    // Customer reference
    #[foreign_key = "customers(id) ON DELETE RESTRICT"]
    #[indexed]
    pub customer_id: uuid::Uuid,

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

    #[default_value = "1.0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub exchange_rate: rust_decimal::Decimal,

    // Payment method
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub payment_method: String, // CHECK, WIRE, ACH, CASH, CREDIT_CARD, etc.

    #[column_type = "VARCHAR(100)"]
    pub payment_reference: Option<String>, // Check number, transaction ID, etc.

    // Bank account
    pub bank_account_id: Option<uuid::Uuid>, // Bank account where payment was received

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, POSTED, RECONCILED, CANCELLED

    // Reconciliation
    pub reconciled_at: Option<chrono::NaiveDateTime>,
    pub reconciled_by: Option<uuid::Uuid>,

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub applied_amount: rust_decimal::Decimal, // Amount applied to invoices

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub unapplied_amount: rust_decimal::Decimal, // Unapplied amount (overpayment)

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
