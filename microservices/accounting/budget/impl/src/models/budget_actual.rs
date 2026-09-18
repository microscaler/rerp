//! Budget Actual entity
//!
//! Actual vs budget comparisons by account and period.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "budget_actuals"]
#[skip_from_row]
#[table_comment = "Actual vs budget comparisons"]
#[index = "idx_budget_actuals_budget_id(budget_id)"]
#[index = "idx_budget_actuals_account_id(account_id)"]
#[index = "idx_budget_actuals_period_id(period_id)"]
#[composite_unique = "budget_id, account_id, period_id"]
pub struct BudgetActual {
    #[primary_key]
    pub id: uuid::Uuid,

    // References
    #[foreign_key = "budgets(id) ON DELETE CASCADE"]
    #[indexed]
    pub budget_id: uuid::Uuid,

    #[foreign_key = "accounts(id) ON DELETE RESTRICT"]
    #[indexed]
    pub account_id: uuid::Uuid,

    #[foreign_key = "budget_periods(id) ON DELETE RESTRICT"]
    #[indexed]
    pub period_id: uuid::Uuid,

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub budget_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub actual_amount: rust_decimal::Decimal,

    // Variance analysis
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub variance: rust_decimal::Decimal, // actual - budget

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub variance_percent: rust_decimal::Decimal, // (variance / budget) * 100

    // Variance flags
    #[default_value = "false"]
    pub is_favorable: bool, // True if variance is favorable (e.g., lower expense than budget)

    #[default_value = "false"]
    pub exceeds_threshold: bool, // True if variance exceeds alert threshold

    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Last calculated
    pub last_calculated_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
