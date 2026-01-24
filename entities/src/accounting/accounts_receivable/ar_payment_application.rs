//! AR Payment Application entity
//!
//! Links payments to specific invoices with applied amounts.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "ar_payment_applications"]
#[skip_from_row]
#[table_comment = "Links AR payments to customer invoices"]
#[index = "idx_ar_payment_applications_payment_id(payment_id)"]
#[index = "idx_ar_payment_applications_invoice_id(invoice_id)"]
#[composite_unique = "payment_id, invoice_id"]
pub struct ArPaymentApplication {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Payment reference
    #[foreign_key = "ar_payments(id) ON DELETE CASCADE"]
    #[indexed]
    pub payment_id: uuid::Uuid,
    
    // Invoice reference
    #[foreign_key = "customer_invoices(id) ON DELETE CASCADE"]
    #[indexed]
    pub invoice_id: uuid::Uuid,
    
    // Applied amount
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub applied_amount: rust_decimal::Decimal,
    
    // Application date
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub applied_at: chrono::NaiveDateTime,
    
    pub applied_by: Option<uuid::Uuid>,
    
    // Notes
    pub notes: Option<String>,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
