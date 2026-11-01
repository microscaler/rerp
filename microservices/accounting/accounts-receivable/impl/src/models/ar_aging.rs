//! Aging snapshot row (as-of reproducible aggregates).
//!
//! Producers land in F3 (aging & statements); this slice delivers only the
//! storage contract. Buckets derive from open-item due dates at snapshot
//! time — never a live-cache substitute for read-time derivation.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "ar_agings"]
#[table_comment = "As-of AR aging snapshots per customer and legal entity"]
#[composite_unique = "tenant_id, legal_entity_id, customer_id, aging_date"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_ar_agings_scope(tenant_id, legal_entity_id)"]
#[index = "idx_ar_agings_customer(customer_id)"]
#[index = "idx_ar_agings_aging_date(aging_date)"]
pub struct ArAging {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[indexed]
    pub customer_id: uuid::Uuid,

    pub aging_date: chrono::NaiveDate,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub current: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub days_31_60: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub days_61_90: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub days_91_120: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub over_120: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub total_outstanding: rust_decimal::Decimal,

    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
