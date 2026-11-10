//! Table `contacts` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! People the workspace can reach. Email unique per workspace (case-insensitive).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "contacts"]
#[schema_name = "audience"]
#[table_comment = "People the workspace can reach. Email unique per workspace (case-insensitive)"]
#[index = "UNIQUE idx_contacts_workspace_id_email(workspace_id, lower(email) | email)"]
#[index = "idx_contacts_status(status)"]
#[index = "idx_contacts_last_activity_at(last_activity_at)"]
#[index = "idx_contacts_created_at(created_at)"]
pub struct Contact {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[column_type = "VARCHAR(255)"]
    pub email: String,

    #[column_type = "VARCHAR(128)"]
    pub first_name: Option<String>,

    #[column_type = "VARCHAR(128)"]
    pub last_name: Option<String>,

    #[column_type = "VARCHAR(32)"]
    pub phone: Option<String>,

    #[column_type = "VARCHAR(128)"]
    pub job_title: Option<String>,

    #[foreign_key = "audience.organisations(id) ON DELETE SET NULL"]
    #[indexed]
    pub organisation_id: Option<uuid::Uuid>,

    #[column_type = "CHAR(2)"]
    pub country_code: Option<String>,

    pub timezone: Option<String>,

    #[column_type = "JSONB"]
    #[default_expr = "'{}'::jsonb"]
    pub custom_fields: serde_json::Value,

    pub source: Option<String>,

    #[column_type = "JSONB"]
    pub attribution: Option<serde_json::Value>,

    #[default_expr = "false"]
    pub sms_consent: bool,

    #[default_expr = "false"]
    pub tracking_optout: bool,

    #[column_type = "TIMESTAMPTZ"]
    pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    #[default_expr = "'PENDING'"]
    #[check = "status IN ('SUBSCRIBED', 'UNSUBSCRIBED', 'PENDING', 'BOUNCED')"]
    pub status: String,

    #[column_type = "TIMESTAMPTZ"]
    pub sms_consent_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub erased_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "VARCHAR(64)"]
    pub created_by: Option<String>,
}
