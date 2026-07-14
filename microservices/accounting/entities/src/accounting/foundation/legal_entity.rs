//! Tenant-scoped legal entity which owns books and document sequences.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_legal_entities"]
#[table_comment = "Tenant-scoped legal entities owning accounting books"]
#[composite_unique = "tenant_id, organization_id"]
#[composite_unique = "tenant_id, id"]
#[index = "idx_accounting_legal_entities_tenant(tenant_id)"]
#[index = "idx_accounting_legal_entities_organization(organization_id)"]
pub struct AccountingLegalEntity {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub organization_id: uuid::Uuid,

    #[column_type = "VARCHAR(255)"]
    pub legal_name: String,

    #[column_type = "VARCHAR(3)"]
    pub base_currency: String,

    #[default_value = "true"]
    pub is_active: bool,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
