//! Table `contact_notes` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Notes on a contact.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "contact_notes"]
#[schema_name = "audience"]
#[table_comment = "Notes on a contact"]
#[index = "idx_contact_notes_contact_id_created_at(contact_id, created_at)"]
pub struct ContactNote {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[foreign_key = "audience.contacts(id) ON DELETE CASCADE"]
    #[indexed]
    pub contact_id: uuid::Uuid,

    pub body: String,

    #[default_expr = "false"]
    pub pinned: bool,

    #[column_type = "VARCHAR(64)"]
    pub created_by: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
