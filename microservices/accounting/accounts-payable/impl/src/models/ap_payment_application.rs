//! AP Payment Application entity
//!
//! Links payments to specific vendor invoices.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "ap_payment_applications"]
#[skip_from_row]
#[table_comment = "Links AP payments to vendor invoices"]
#[index = "idx_ap_payment_applications_payment_id(payment_id)"]
#[index = "idx_ap_payment_applications_invoice_id(invoice_id)"]
#[composite_unique = "payment_id, invoice_id"]
pub struct ApPaymentApplication {
    #[primary_key]
    pub id: uuid::Uuid,

    #[foreign_key = "ap_payments(id) ON DELETE CASCADE"]
    #[indexed]
    pub payment_id: uuid::Uuid,

    #[foreign_key = "vendor_invoices(id) ON DELETE CASCADE"]
    #[indexed]
    pub invoice_id: uuid::Uuid,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub applied_amount: rust_decimal::Decimal,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub applied_at: chrono::NaiveDateTime,

    pub applied_by: Option<uuid::Uuid>,

    pub notes: Option<String>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
