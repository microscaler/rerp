//! Budget entity
//!
//! Budget definitions for planning and control.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "budgets"]
#[skip_from_row]
#[table_comment = "Budget definitions"]
#[index = "idx_budgets_budget_number(budget_number)"]
#[index = "idx_budgets_fiscal_year(fiscal_year)"]
#[index = "idx_budgets_status(status)"]
#[index = "idx_budgets_company_id(company_id)"]
pub struct Budget {
    #[primary_key]
    pub id: uuid::Uuid,

    // Budget identification
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub budget_number: String,

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    pub description: Option<String>,

    // Budget period
    #[indexed]
    pub fiscal_year: i32,

    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,

    // Version tracking
    pub current_version_id: Option<uuid::Uuid>, // Current active version

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, ACTIVE, CLOSED, CANCELLED

    // Approval workflow
    #[column_type = "VARCHAR(50)"]
    pub approval_status: Option<String>, // PENDING, APPROVED, REJECTED

    pub approved_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<uuid::Uuid>,

    // Totals (calculated from line items)
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_budget_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_actual_amount: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_variance: rust_decimal::Decimal, // actual - budget

    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Multi-company
    #[indexed]
    pub company_id: Option<uuid::Uuid>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
