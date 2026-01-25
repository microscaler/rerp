//! Chart of Accounts entity
//!
//! This entity represents the hierarchical chart of accounts structure.
//! It's a self-referencing table where accounts can have parent accounts.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "chart_of_accounts"]
#[skip_from_row] // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Hierarchical chart of accounts structure"]
#[index = "idx_chart_of_accounts_code(code)"]
#[index = "idx_chart_of_accounts_parent_id(parent_id)"]
#[index = "idx_chart_of_accounts_account_type(account_type)"]
#[index = "idx_chart_of_accounts_is_active(is_active)"]
pub struct ChartOfAccount {
    #[primary_key]
    pub id: uuid::Uuid,

    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub code: String,

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub account_type: String, // ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE

    // Self-referencing foreign key
    #[foreign_key = "chart_of_accounts(id) ON DELETE SET NULL"]
    #[indexed]
    pub parent_id: Option<uuid::Uuid>,

    #[default_value = "0"]
    pub level: i32, // Hierarchy level (0 = root)

    #[default_value = "true"]
    #[indexed]
    pub is_active: bool,

    pub description: Option<String>,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
