//! Append-only audit fact for an accounting state transition.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_audit_events"]
#[table_comment = "Append-only accounting transition audit facts"]
#[index = "idx_accounting_audit_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_audit_document(document_id)"]
#[index = "idx_accounting_audit_time(occurred_at)"]
pub struct AccountingAuditEvent {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    pub subject_id: uuid::Uuid,

    #[column_type = "VARCHAR(50)"]
    pub action: String,

    #[indexed]
    pub document_id: uuid::Uuid,

    pub original_document_id: Option<uuid::Uuid>,

    #[column_type = "VARCHAR(100)"]
    pub source_system: String,

    #[column_type = "VARCHAR(64)"]
    pub request_fingerprint: String,

    #[indexed]
    pub occurred_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub recorded_at: chrono::NaiveDateTime,
}
