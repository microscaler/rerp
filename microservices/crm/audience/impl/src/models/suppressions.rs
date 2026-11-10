//! Table `suppressions` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Addresses no list may message (hard bounce, complaint, manual, erased).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "suppressions"]
#[schema_name = "audience"]
#[table_comment = "Addresses no list may message (hard bounce, complaint, manual, erased)"]
#[index = "UNIQUE idx_suppressions_workspace_id_channel_address(workspace_id, channel, lower(address) | address)"]
pub struct Suppression {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    pub address: String,

    #[default_expr = "'EMAIL'"]
    #[check = "channel IN ('EMAIL', 'SMS')"]
    pub channel: String,

    #[check = "reason IN ('HARD_BOUNCE', 'COMPLAINT', 'MANUAL', 'GLOBAL_UNSUBSCRIBE')"]
    pub reason: String,

    pub note: Option<String>,

    pub source_send_id: Option<uuid::Uuid>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub source: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
