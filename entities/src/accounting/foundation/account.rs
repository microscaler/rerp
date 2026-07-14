//! Tenant-scoped chart account used by the Phase 1 posting kernel.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_accounts"]
#[table_comment = "Posting accounts for the controlled accounting core"]
#[composite_unique = "tenant_id, legal_entity_id, code"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_accounts_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_accounts_type(account_type)"]
#[index = "idx_accounting_accounts_control_role(control_role)"]
pub struct AccountingAccount {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[foreign_key = "accounting_legal_entities(id) ON DELETE RESTRICT"]
    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[column_type = "VARCHAR(50)"]
    pub code: String,

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    #[indexed]
    #[column_type = "VARCHAR(20)"]
    pub account_type: String,

    #[column_type = "VARCHAR(6)"]
    pub normal_side: String,

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub control_role: Option<String>,

    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    #[default_value = "true"]
    pub is_active: bool,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
