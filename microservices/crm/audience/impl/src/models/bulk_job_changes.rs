//! Table `bulk_job_changes` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Before and after values per contact, for undo.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "bulk_job_changes"]
#[schema_name = "audience"]
#[table_comment = "Before and after values per contact, for undo"]
pub struct BulkJobChange {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    #[foreign_key = "audience.bulk_jobs(id) ON DELETE CASCADE"]
    #[indexed]
    pub job_id: uuid::Uuid,

    #[primary_key]
    pub contact_id: uuid::Uuid,

    #[column_type = "JSONB"]
    pub before: serde_json::Value,

    #[column_type = "JSONB"]
    pub after: serde_json::Value,
}
