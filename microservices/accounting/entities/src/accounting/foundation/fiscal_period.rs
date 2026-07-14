//! Fiscal period controls for posting and close.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_fiscal_periods"]
#[table_comment = "Tenant and legal-entity scoped fiscal posting periods"]
#[check = "valid_period_dates: start_date <= end_date"]
#[composite_unique = "tenant_id, legal_entity_id, name"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_fiscal_periods_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_fiscal_periods_dates(start_date, end_date)"]
#[index = "idx_accounting_fiscal_periods_state(state)"]
pub struct AccountingFiscalPeriod {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[foreign_key = "accounting_legal_entities(id) ON DELETE RESTRICT"]
    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub name: String,

    #[indexed]
    pub start_date: chrono::NaiveDate,

    #[indexed]
    pub end_date: chrono::NaiveDate,

    #[default_value = "'OPEN'"]
    #[indexed]
    #[column_type = "VARCHAR(20)"]
    pub state: String,

    pub closed_at: Option<chrono::NaiveDateTime>,

    pub closed_by: Option<uuid::Uuid>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
