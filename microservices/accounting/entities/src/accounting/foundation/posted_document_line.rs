//! Immutable commercial and tax snapshot for one posted document line.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_posted_document_lines"]
#[table_comment = "Immutable line snapshots for posted accounting documents"]
#[composite_unique = "tenant_id, legal_entity_id, document_id, line_number"]
#[index = "idx_accounting_document_lines_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_document_lines_document(document_id)"]
pub struct AccountingPostedDocumentLine {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[foreign_key = "accounting_posted_documents(id) ON DELETE RESTRICT"]
    #[indexed]
    pub document_id: uuid::Uuid,

    pub line_number: i32,

    #[column_type = "VARCHAR(1000)"]
    pub description: String,

    #[column_type = "NUMERIC(19, 6)"]
    pub quantity: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub unit_price: rust_decimal::Decimal,

    #[column_type = "NUMERIC(9, 6)"]
    pub discount_percent: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub gross_amount: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub discount_amount: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub net_amount: rust_decimal::Decimal,

    #[column_type = "VARCHAR(100)"]
    pub tax_code: Option<String>,

    #[column_type = "NUMERIC(9, 6)"]
    pub tax_rate_percent: Option<rust_decimal::Decimal>,

    #[column_type = "NUMERIC(19, 6)"]
    pub tax_amount: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub total_amount: rust_decimal::Decimal,

    #[foreign_key = "accounting_accounts(id) ON DELETE RESTRICT"]
    pub revenue_account_id: uuid::Uuid,

    pub tax_liability_account_id: Option<uuid::Uuid>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
