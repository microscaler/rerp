//! Table `contact_facts` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Per-contact projection of facts from other services, so a segment is one query.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "contact_facts"]
#[schema_name = "audience"]
#[table_comment = "Per-contact projection of facts from other services, so a segment is one query"]
pub struct ContactFact {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    #[foreign_key = "audience.contacts(id) ON DELETE CASCADE"]
    #[indexed]
    pub contact_id: uuid::Uuid,

    #[column_type = "TIMESTAMPTZ"]
    pub last_opened_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub last_clicked_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub last_sent_at: Option<chrono::DateTime<chrono::Utc>>,

    #[default_expr = "'0'"]
    pub opens_30d: i32,

    #[default_expr = "'0'"]
    pub clicks_30d: i32,

    #[column_type = "JSONB"]
    pub pages_visited_30d: Option<serde_json::Value>,

    #[column_type = "JSONB"]
    pub events_30d: Option<serde_json::Value>,

    #[column_type = "JSONB"]
    pub forms_submitted: Option<serde_json::Value>,

    #[column_type = "JSONB"]
    pub deal_stage_ids: Option<serde_json::Value>,

    #[default_expr = "'0'"]
    pub open_deal_count: i32,

    #[column_type = "JSONB"]
    pub score_bands: Option<serde_json::Value>,

    #[column_type = "JSONB"]
    pub active_workflow_ids: Option<serde_json::Value>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
