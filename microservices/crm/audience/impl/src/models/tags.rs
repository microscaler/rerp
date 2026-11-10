//! Table `tags` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Tag definitions; names unique per workspace.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "tags"]
#[schema_name = "audience"]
#[table_comment = "Tag definitions; names unique per workspace"]
#[index = "UNIQUE idx_tags_workspace_id_name(workspace_id, lower(name) | name)"]
pub struct Tag {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[column_type = "VARCHAR(64)"]
    pub name: String,

    #[column_type = "CHAR(7)"]
    pub color: Option<String>,

    pub description: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
