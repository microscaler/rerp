//! Table `bulk_jobs` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Changes applied to many contacts; undoable for 7 days.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "bulk_jobs"]
#[schema_name = "audience"]
#[table_comment = "Changes applied to many contacts; undoable for 7 days"]
pub struct BulkJob {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    pub label: Option<String>,

    #[check = "action IN ('ADD_TAG', 'REMOVE_TAG', 'SUBSCRIBE', 'UNSUBSCRIBE', 'SET_FIELD', 'ARCHIVE')"]
    pub action: String,

    pub value: Option<String>,

    pub field_key: Option<String>,

    #[foreign_key = "audience.lists(id) ON DELETE SET NULL"]
    #[indexed]
    pub list_id: Option<uuid::Uuid>,

    #[foreign_key = "audience.segments(id) ON DELETE SET NULL"]
    #[indexed]
    pub segment_id: Option<uuid::Uuid>,

    pub contact_total: Option<i64>,

    #[check = "status IN ('QUEUED', 'RUNNING', 'DONE', 'FAILED', 'UNDONE')"]
    pub status: String,

    pub started_by: Option<String>,

    pub undoable: Option<bool>,

    #[column_type = "TIMESTAMPTZ"]
    pub undo_until: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub undone_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
