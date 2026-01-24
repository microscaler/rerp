//! Account entity
//!
//! Individual accounts linked to chart of accounts.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "accounts"]
#[skip_from_row]  // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Individual accounts linked to chart of accounts"]
#[index = "idx_accounts_chart_of_account_id(chart_of_account_id)"]
#[index = "idx_accounts_code(code)"]
#[index = "idx_accounts_account_type(account_type)"]
#[index = "idx_accounts_is_active(is_active)"]
#[index = "idx_accounts_currency_code(currency_code)"]
pub struct Account {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Foreign key to chart_of_accounts
    #[foreign_key = "chart_of_accounts(id) ON DELETE RESTRICT"]
    #[indexed]
    pub chart_of_account_id: uuid::Uuid,
    
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub code: String,
    
    #[column_type = "VARCHAR(255)"]
    pub name: String,
    
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub account_type: String, // ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE
    
    #[column_type = "VARCHAR(10)"]
    pub normal_balance: String, // DEBIT or CREDIT
    
    #[default_value = "'USD'"]
    #[indexed]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[default_value = "true"]
    #[indexed]
    pub is_active: bool,
    
    #[default_value = "false"]
    pub is_system_account: bool, // System accounts cannot be deleted
    
    pub description: Option<String>,
    
    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // JSONB
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
