//! Durable render-outbox instruction for a posted accounting document
//! (NFR-DOG-010): the immutable render model and the render directive are
//! stored atomically with the posting, so post-commit rendering is
//! durable, retryable and operator-inspectable while rendering is down.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_render_outbox"]
#[table_comment = "Durable render instructions with the immutable render model captured at posting time"]
#[composite_unique = "tenant_id, legal_entity_id, document_id, media_type"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_render_outbox_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_render_outbox_status(status)"]
#[index = "idx_accounting_render_outbox_document(document_id)"]
pub struct AccountingRenderOutbox {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[indexed]
    pub document_id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub media_type: String,

    #[column_type = "VARCHAR(30)"]
    pub renderer: String,

    #[column_type = "VARCHAR(30)"]
    pub renderer_version: String,

    #[column_type = "JSONB"]
    pub render_model: serde_json::Value,

    #[column_type = "VARCHAR(20)"]
    pub status: String,

    pub attempts: i32,

    #[column_type = "VARCHAR(500)"]
    pub last_error: Option<String>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    pub updated_at: chrono::NaiveDateTime,
}
