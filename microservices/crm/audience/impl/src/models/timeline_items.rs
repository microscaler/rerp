//! Table `timeline_items` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! The contact timeline, projected from events of every service.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "timeline_items"]
#[schema_name = "audience"]
#[table_comment = "The contact timeline, projected from events of every service"]
#[index = "idx_timeline_items_contact_id_at(contact_id, at DESC)"]
pub struct TimelineItem {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[foreign_key = "audience.contacts(id) ON DELETE CASCADE"]
    #[indexed]
    pub contact_id: uuid::Uuid,

    #[check = "kind IN ('MESSAGE_SENT', 'MESSAGE_OPENED', 'MESSAGE_CLICKED', 'MESSAGE_BOUNCED', 'UNSUBSCRIBED', 'SUBSCRIBED', 'FORM_SUBMITTED', 'PAGE_VIEW', 'SITE_EVENT', 'WORKFLOW_ENTERED', 'WORKFLOW_STEP', 'WORKFLOW_ENDED', 'TAG_ADDED', 'TAG_REMOVED', 'FIELD_CHANGED', 'SCORE_CHANGED', 'DEAL_CREATED', 'DEAL_STAGE_CHANGED', 'NOTE', 'CHAT')"]
    pub kind: String,

    #[column_type = "TIMESTAMPTZ"]
    pub at: chrono::DateTime<chrono::Utc>,

    pub summary: String,

    pub actor: Option<String>,

    pub ref_service: Option<String>,

    pub ref_id: Option<uuid::Uuid>,

    #[column_type = "JSONB"]
    pub data: Option<serde_json::Value>,
}
