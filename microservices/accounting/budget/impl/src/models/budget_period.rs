//! Budget Period entity
//!
//! Time periods for budget tracking (months, quarters, etc.).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "budget_periods"]
#[skip_from_row]
#[table_comment = "Budget time periods"]
#[index = "idx_budget_periods_period_start(period_start)"]
#[index = "idx_budget_periods_period_type(period_type)"]
#[composite_unique = "period_start, period_end, period_type"]
pub struct BudgetPeriod {
    #[primary_key]
    pub id: uuid::Uuid,

    // Period identification
    #[column_type = "VARCHAR(50)"]
    pub period_name: String, // e.g., "2024-Q1", "2024-01"

    #[indexed]
    pub period_start: chrono::NaiveDate,

    pub period_end: chrono::NaiveDate,

    // Period type
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub period_type: String, // MONTH, QUARTER, YEAR, CUSTOM

    // Fiscal year
    pub fiscal_year: i32,

    #[default_value = "1"]
    pub period_number: i32, // Period number within fiscal year (1-12 for months, 1-4 for quarters)

    pub description: Option<String>,

    #[default_value = "true"]
    pub is_active: bool,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
