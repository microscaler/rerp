//! Table `segments` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Saved segments: a two-level condition tree over the contact facts.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "segments"]
#[schema_name = "audience"]
#[table_comment = "Saved segments: a two-level condition tree over the contact facts"]
pub struct Segment {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[column_type = "VARCHAR(128)"]
    pub name: String,

    pub description: Option<String>,

    #[column_type = "JSONB"]
    pub conditions: serde_json::Value,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub contact_count: Option<i64>,

    #[column_type = "TIMESTAMPTZ"]
    pub counted_at: Option<chrono::DateTime<chrono::Utc>>,
}
