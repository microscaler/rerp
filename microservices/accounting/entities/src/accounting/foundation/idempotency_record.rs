//! Tenant-scoped idempotency ownership and completed result link.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_idempotency_records"]
#[table_comment = "Accounting command idempotency and payload conflict records"]
#[composite_unique = "tenant_id, legal_entity_id, idempotency_key"]
#[index = "idx_accounting_idempotency_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_idempotency_source(source_system, source_type, source_id)"]
pub struct AccountingIdempotencyRecord {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[column_type = "VARCHAR(200)"]
    pub idempotency_key: String,

    #[column_type = "VARCHAR(64)"]
    pub request_fingerprint: String,

    #[column_type = "VARCHAR(20)"]
    pub status: String,

    #[column_type = "VARCHAR(100)"]
    pub source_system: String,

    #[column_type = "VARCHAR(100)"]
    pub source_type: String,

    #[column_type = "VARCHAR(255)"]
    pub source_id: String,

    pub document_id: Option<uuid::Uuid>,

    pub journal_entry_id: Option<uuid::Uuid>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    pub completed_at: Option<chrono::NaiveDateTime>,
}
