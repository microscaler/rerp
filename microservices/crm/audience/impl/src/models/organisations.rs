//! Table `organisations` (design: `openapi/crm/docs/data-model/audience.yaml`).
//!
//! Prospect companies. A customer exists only in RERP crm/accounts (ADR 003).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "organisations"]
#[schema_name = "audience"]
#[table_comment = "Prospect companies. A customer exists only in RERP crm/accounts (ADR 003)"]
#[index = "UNIQUE idx_organisations_workspace_id_domain(workspace_id, lower(domain) | domain)"]
#[index = "idx_organisations_name(name)"]
pub struct Organisation {
    pub workspace_id: uuid::Uuid,

    #[primary_key]
    pub id: uuid::Uuid,

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    #[column_type = "VARCHAR(255)"]
    pub domain: Option<String>,

    pub website: Option<String>,

    #[column_type = "VARCHAR(128)"]
    pub industry: Option<String>,

    #[check = "size IN ('1-10', '11-50', '51-200', '201-1000', '1001-5000', '5000+')"]
    pub size: Option<String>,

    #[column_type = "CHAR(2)"]
    pub country_code: Option<String>,

    #[foreign_key = "audience.organisations(id) ON DELETE SET NULL"]
    #[indexed]
    pub parent_id: Option<uuid::Uuid>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[column_type = "TIMESTAMPTZ"]
    #[default_expr = "now()"]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub customer_account_id: Option<uuid::Uuid>,

    #[column_type = "TIMESTAMPTZ"]
    pub converted_at: Option<chrono::DateTime<chrono::Utc>>,

    #[column_type = "TIMESTAMPTZ"]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
}
