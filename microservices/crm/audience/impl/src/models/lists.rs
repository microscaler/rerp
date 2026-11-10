//! Table `lists` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Mailing lists (email or SMS) with their consent settings.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "lists"]
#[schema_name = "audience"]
#[table_comment = "Mailing lists (email or SMS) with their consent settings"]
#[index = "UNIQUE idx_lists_workspace_id_name(workspace_id, lower(name) | name)"]
pub struct MailingList {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[column_type = "VARCHAR(128)"]
    pub name: String,

    pub description: Option<String>,

    pub sender_id: uuid::Uuid,

    pub double_opt_in: bool,

    pub confirmation_template_id: Option<uuid::Uuid>,

    #[default_expr = "false"]
    pub is_archived: bool,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    #[default_expr = "'EMAIL'"]
    #[check = "channel IN ('EMAIL', 'SMS')"]
    pub channel: String,

    pub postal_address: Option<String>,

    pub reminder_text: Option<String>,
}
