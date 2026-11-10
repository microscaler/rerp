//! Table `subscriptions` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! One contact on one list, with the consent that put them there.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "subscriptions"]
#[schema_name = "audience"]
#[table_comment = "One contact on one list, with the consent that put them there"]
#[index = "UNIQUE idx_subscriptions_list_id_contact_id(list_id, contact_id)"]
#[index = "idx_subscriptions_status(status)"]
pub struct Subscription {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[foreign_key = "audience.contacts(id) ON DELETE CASCADE"]
    #[indexed]
    pub contact_id: uuid::Uuid,

    #[foreign_key = "audience.lists(id) ON DELETE CASCADE"]
    #[indexed]
    pub list_id: uuid::Uuid,

    #[check = "status IN ('PENDING', 'SUBSCRIBED', 'UNSUBSCRIBED', 'BOUNCED', 'COMPLAINED')"]
    pub status: String,

    #[check = "consent_source IN ('WEB_FORM', 'DOUBLE_OPT_IN', 'IMPORT', 'STAFF', 'API', 'PREFERENCE_PAGE', 'CHECKOUT')"]
    pub consent_source: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    pub consent_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "VARCHAR(45)"]
    pub consent_ip: Option<String>,

    pub consent_text: Option<String>,

    pub form_id: Option<uuid::Uuid>,

    #[column_type = "TIMESTAMPTZ"]
    pub unsubscribed_at: Option<chrono::DateTime<chrono::Utc>>,

    pub unsubscribe_reason: Option<String>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
