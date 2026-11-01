//! Allocation linking one payment to one AR overlay row.
//!
//! Write path and reversal semantics land in F2; this slice delivers only
//! the storage contract with tenant-scoped identity.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "ar_payment_applications"]
#[table_comment = "Links AR payments to customer-invoice overlay rows"]
#[composite_unique = "tenant_id, legal_entity_id, payment_id, invoice_id"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_ar_payment_applications_scope(tenant_id, legal_entity_id)"]
#[index = "idx_ar_payment_applications_payment(payment_id)"]
#[index = "idx_ar_payment_applications_invoice(invoice_id)"]
pub struct ArPaymentApplication {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[indexed]
    pub payment_id: uuid::Uuid,

    #[indexed]
    pub invoice_id: uuid::Uuid,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
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
