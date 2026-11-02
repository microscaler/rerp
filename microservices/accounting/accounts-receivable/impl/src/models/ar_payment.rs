//! Customer payment header.
//!
//! Receipts are authoritative external facts (external payment reference);
//! F2 delivers the write path. Amounts follow the decimal-string contract
//! (NUMERIC(19,6) like the posted facts).

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "ar_payments"]
#[table_comment = "Customer payments with external reference and allocation totals"]
#[composite_unique = "tenant_id, legal_entity_id, payment_number"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_ar_payments_scope(tenant_id, legal_entity_id)"]
#[index = "idx_ar_payments_customer(customer_id)"]
#[index = "idx_ar_payments_payment_date(payment_date)"]
#[index = "idx_ar_payments_status(status)"]
pub struct ArPayment {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[indexed]
    pub customer_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub payment_number: String,

    pub payment_date: chrono::NaiveDate,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub payment_amount: rust_decimal::Decimal,

    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    #[default_value = "1.0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub exchange_rate: rust_decimal::Decimal,

    /// CHECK, WIRE, ACH, CASH, CREDIT_CARD, ...
    #[column_type = "VARCHAR(50)"]
    pub payment_method: String,

    /// External provider reference (authoritative, F2 duplicate guard).
    /// Partial unique (tenant, legal_entity_id) lands as hand-written
    /// foundation/0007_ar_receipts.sql — the generator has no partial index.
    #[column_type = "VARCHAR(100)"]
    pub payment_reference: Option<String>,

    /// Receiving bank account; registry lands in Epic H1 (no FK yet).
    pub bank_account_id: Option<uuid::Uuid>,

    /// DRAFT, POSTED, RECONCILED, CANCELLED.
    #[default_value = "'DRAFT'"]
    #[column_type = "VARCHAR(50)"]
    pub status: String,

    pub reconciled_at: Option<chrono::NaiveDateTime>,
    pub reconciled_by: Option<uuid::Uuid>,

    /// Sum of live allocations; maintained by the F2 write path.
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub applied_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub unapplied_amount: rust_decimal::Decimal,

    pub notes: Option<String>,

    #[column_type = "JSONB"]
    pub metadata: Option<serde_json::Value>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
