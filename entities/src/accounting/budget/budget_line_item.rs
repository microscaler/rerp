//! Budget Line Item entity
//!
//! Individual line items in budgets with account and period breakdowns.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "budget_line_items"]
#[skip_from_row]
#[table_comment = "Budget line items by account and period"]
#[index = "idx_budget_line_items_budget_id(budget_id)"]
#[index = "idx_budget_line_items_version_id(version_id)"]
#[index = "idx_budget_line_items_account_id(account_id)"]
#[index = "idx_budget_line_items_period_id(period_id)"]
#[composite_unique = "budget_id, version_id, account_id, period_id"]
pub struct BudgetLineItem {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Budget and version references
    #[foreign_key = "budgets(id) ON DELETE CASCADE"]
    #[indexed]
    pub budget_id: uuid::Uuid,
    
    #[foreign_key = "budget_versions(id) ON DELETE CASCADE"]
    #[indexed]
    pub version_id: uuid::Uuid,
    
    // Account reference
    #[foreign_key = "accounts(id) ON DELETE RESTRICT"]
    #[indexed]
    pub account_id: uuid::Uuid,
    
    // Period reference
    #[foreign_key = "budget_periods(id) ON DELETE RESTRICT"]
    #[indexed]
    pub period_id: uuid::Uuid,
    
    // Budget amount
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub budget_amount: rust_decimal::Decimal,
    
    // Actual amount (calculated from transactions)
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub actual_amount: rust_decimal::Decimal,
    
    // Variance
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub variance: rust_decimal::Decimal, // actual_amount - budget_amount
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub variance_percent: rust_decimal::Decimal, // (variance / budget_amount) * 100
    
    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // Notes
    pub notes: Option<String>,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
