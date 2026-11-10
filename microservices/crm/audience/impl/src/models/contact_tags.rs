//! Table `contact_tags` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Which contacts carry which tags.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "contact_tags"]
#[schema_name = "audience"]
#[table_comment = "Which contacts carry which tags"]
pub struct ContactTag {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    #[foreign_key = "audience.contacts(id) ON DELETE CASCADE"]
    #[indexed]
    pub contact_id: uuid::Uuid,

    #[primary_key]
    #[foreign_key = "audience.tags(id) ON DELETE CASCADE"]
    #[indexed]
    pub tag_id: uuid::Uuid,

    #[default_expr = "'USER'"]
    #[check = "source IN ('USER', 'AUTOMATION', 'BULK_JOB', 'IMPORT', 'FORM')"]
    pub source: String,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
