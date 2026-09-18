//! Immutable metadata for a rendered accounting document stored outside PostgreSQL.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_document_artifacts"]
#[table_comment = "Immutable content-addressed rendered accounting document metadata"]
#[composite_unique = "tenant_id, legal_entity_id, document_id, media_type"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_document_artifacts_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_document_artifacts_document(document_id)"]
#[index = "idx_accounting_document_artifacts_checksum(sha256)"]
pub struct AccountingDocumentArtifact {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[indexed]
    pub document_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub media_type: String,

    #[column_type = "VARCHAR(100)"]
    pub storage_provider: String,

    #[column_type = "VARCHAR(255)"]
    pub bucket: String,

    #[column_type = "VARCHAR(1024)"]
    pub object_key: String,

    #[column_type = "VARCHAR(64)"]
    pub sha256: String,

    pub size_bytes: i64,

    #[column_type = "VARCHAR(30)"]
    pub renderer: String,

    #[column_type = "VARCHAR(30)"]
    pub renderer_version: String,

    pub rendered_at: chrono::NaiveDateTime,

    pub rendered_by: uuid::Uuid,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
