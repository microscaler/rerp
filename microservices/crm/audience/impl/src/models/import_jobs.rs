//! Table `import_jobs` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Contact imports and exports run by audience.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "import_jobs"]
#[schema_name = "audience"]
#[table_comment = "Contact imports and exports run by audience"]
pub struct ImportJob {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[check = "kind IN ('IMPORT', 'EXPORT')"]
    pub kind: String,

    #[check = "status IN ('QUEUED', 'RUNNING', 'DONE', 'FAILED')"]
    pub status: String,

    pub total_rows: Option<i64>,

    pub processed_rows: Option<i64>,

    pub created_rows: Option<i64>,

    pub updated_rows: Option<i64>,

    pub skipped_rows: Option<i64>,

    #[column_type = "JSONB"]
    pub errors: Option<serde_json::Value>,

    pub download_url: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
