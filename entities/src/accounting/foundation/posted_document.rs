//! Immutable posted customer invoice or credit-note header.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_posted_documents"]
#[table_comment = "Immutable posted accounting document headers"]
#[composite_unique = "tenant_id, legal_entity_id, document_number"]
#[composite_unique = "tenant_id, legal_entity_id, source_system, source_type, source_id"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_documents_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_documents_customer(customer_id)"]
#[index = "idx_accounting_documents_source(source_system, source_type, source_id)"]
#[index = "idx_accounting_documents_period(fiscal_period_id)"]
pub struct AccountingPostedDocument {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[foreign_key = "accounting_legal_entities(id) ON DELETE RESTRICT"]
    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[foreign_key = "accounting_fiscal_periods(id) ON DELETE RESTRICT"]
    #[indexed]
    pub fiscal_period_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub document_number: String,

    #[column_type = "VARCHAR(30)"]
    pub document_type: String,

    #[default_value = "'POSTED'"]
    #[column_type = "VARCHAR(20)"]
    pub status: String,

    pub original_document_id: Option<uuid::Uuid>,

    #[indexed]
    pub customer_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub source_system: String,

    #[column_type = "VARCHAR(100)"]
    pub source_type: String,

    #[column_type = "VARCHAR(255)"]
    pub source_id: String,

    pub document_date: chrono::NaiveDate,

    pub due_date: chrono::NaiveDate,

    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    pub rounding_minor_units: i16,

    #[column_type = "NUMERIC(19, 6)"]
    pub subtotal: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub discount_amount: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub tax_amount: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub total_amount: rust_decimal::Decimal,

    pub posted_at: chrono::NaiveDateTime,

    pub posted_by: uuid::Uuid,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
