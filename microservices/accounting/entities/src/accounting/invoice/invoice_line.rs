//! Invoice Line entity
//!
//! Line items on invoices with support for products, services, taxes, and discounts.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "invoice_lines"]
#[skip_from_row] // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Line items on invoices"]
#[index = "idx_invoice_lines_invoice_id(invoice_id)"]
#[index = "idx_invoice_lines_product_id(product_id)"]
#[index = "idx_invoice_lines_account_id(account_id)"]
pub struct InvoiceLine {
    #[primary_key]
    pub id: uuid::Uuid,

    // Foreign key to invoice
    #[foreign_key = "invoices(id) ON DELETE CASCADE"]
    #[indexed]
    pub invoice_id: uuid::Uuid,

    // Line sequence
    #[default_value = "0"]
    pub line_number: i32,

    // Product/Service reference
    pub product_id: Option<uuid::Uuid>, // Reference to product/service
    pub product_code: Option<String>,
    pub product_name: String,
    pub product_description: Option<String>,

    // Account reference (for GL posting)
    pub account_id: Option<uuid::Uuid>, // GL account for this line

    // Quantities
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub quantity: rust_decimal::Decimal,

    #[column_type = "VARCHAR(50)"]
    pub unit_of_measure: Option<String>, // UOM code

    // Pricing
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub unit_price: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub discount_percent: rust_decimal::Decimal, // Percentage discount (0-100)

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub discount_amount: rust_decimal::Decimal, // Fixed discount amount

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub line_subtotal: rust_decimal::Decimal, // quantity * unit_price - discount

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub tax_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub line_total: rust_decimal::Decimal, // line_subtotal + tax_amount

    // Tax information
    pub tax_id: Option<uuid::Uuid>,              // Reference to tax
    pub tax_rate: Option<rust_decimal::Decimal>, // Tax rate percentage

    // Currency (inherited from invoice, but stored for denormalization)
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // JSONB for extensibility

    // Audit fields
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
