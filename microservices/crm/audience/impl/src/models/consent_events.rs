//! Table `consent_events` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Append-only history of each subscription status change (the consent evidence).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "consent_events"]
#[schema_name = "audience"]
#[table_comment = "Append-only history of each subscription status change (the consent evidence)"]
#[index = "idx_consent_events_subscription_id_at(subscription_id, at)"]
pub struct ConsentEvent {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[foreign_key = "audience.subscriptions(id) ON DELETE CASCADE"]
    #[indexed]
    pub subscription_id: uuid::Uuid,

    pub from_status: Option<String>,

    #[check = "to_status IN ('PENDING', 'SUBSCRIBED', 'UNSUBSCRIBED', 'BOUNCED')"]
    pub to_status: String,

    pub source: String,

    pub actor: Option<String>,

    #[column_type = "VARCHAR(45)"]
    pub ip: Option<String>,

    pub note: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub at: chrono::DateTime<chrono::Utc>,
}
