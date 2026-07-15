//! Invoice entity
//!
//! Represents both customer invoices and vendor bills with comprehensive
//! support for multi-currency, taxes, payment terms, and workflow states.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "invoices"]
#[skip_from_row] // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Customer invoices and vendor bills"]
#[index = "idx_invoices_invoice_number(invoice_number)"]
#[index = "idx_invoices_invoice_date(invoice_date)"]
#[index = "idx_invoices_due_date(due_date)"]
#[index = "idx_invoices_invoice_type(invoice_type)"]
#[index = "idx_invoices_status(status)"]
#[index = "idx_invoices_payment_state(payment_state)"]
#[index = "idx_invoices_customer_id(customer_id)"]
#[index = "idx_invoices_vendor_id(vendor_id)"]
#[index = "idx_invoices_company_id(company_id)"]
#[index = "idx_invoices_currency_code(currency_code)"]
pub struct Invoice {
    #[primary_key]
    pub id: uuid::Uuid,

    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub invoice_number: String,

    #[indexed]
    pub invoice_date: chrono::NaiveDate,

    #[indexed]
    pub due_date: Option<chrono::NaiveDate>,

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub invoice_type: String, // CUSTOMER_INVOICE, VENDOR_BILL, CREDIT_NOTE, REFUND

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, POSTED, PAID, CANCELLED, REVERSED

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub payment_state: String, // NOT_PAID, PARTIAL, PAID, OVERPAID, REVERSED

    // Customer/Vendor reference
    pub customer_id: Option<uuid::Uuid>, // For customer invoices
    pub vendor_id: Option<uuid::Uuid>,   // For vendor bills

    // Payment terms
    pub payment_term_id: Option<uuid::Uuid>, // Reference to payment terms

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub subtotal: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub tax_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub discount_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub paid_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub outstanding_amount: rust_decimal::Decimal, // total_amount - paid_amount

    // Currency
    #[default_value = "'USD'"]
    #[indexed]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    #[default_value = "1.0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub exchange_rate: rust_decimal::Decimal,

    // Reference numbers
    #[column_type = "VARCHAR(100)"]
    pub reference_number: Option<String>, // External reference (PO number, etc.)

    #[column_type = "VARCHAR(100)"]
    pub vendor_reference: Option<String>, // Vendor's invoice number

    // Dates
    pub posted_at: Option<chrono::NaiveDateTime>,
    pub paid_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,

    // Users
    pub posted_by: Option<uuid::Uuid>,
    pub paid_by: Option<uuid::Uuid>,
    pub cancelled_by: Option<uuid::Uuid>,

    // Multi-company support
    #[indexed]
    pub company_id: Option<uuid::Uuid>,

    // Notes and metadata
    pub notes: Option<String>,
    pub internal_notes: Option<String>,

    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // JSONB for extensibility

    // Audit fields
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
