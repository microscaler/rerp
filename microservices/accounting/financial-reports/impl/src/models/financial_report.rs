//! Financial Report entity
//!
//! Financial report definitions (P&L, Balance Sheet, Cash Flow, etc.).

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "financial_reports"]
#[skip_from_row]
#[table_comment = "Financial report definitions"]
#[index = "idx_financial_reports_report_code(report_code)"]
#[index = "idx_financial_reports_report_type(report_type)"]
#[index = "idx_financial_reports_status(status)"]
pub struct FinancialReport {
    #[primary_key]
    pub id: uuid::Uuid,

    // Report identification
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub report_code: String, // e.g., "PL", "BS", "CF"

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    pub description: Option<String>,

    // Report type
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub report_type: String, // PROFIT_LOSS, BALANCE_SHEET, CASH_FLOW, TRIAL_BALANCE, CUSTOM

    // Template reference
    pub template_id: Option<uuid::Uuid>, // Report template used

    // Report period
    pub report_date: Option<chrono::NaiveDate>, // Report as-of date
    pub period_start: Option<chrono::NaiveDate>,
    pub period_end: Option<chrono::NaiveDate>,

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, GENERATED, APPROVED, PUBLISHED

    pub generated_at: Option<chrono::NaiveDateTime>,
    pub generated_by: Option<uuid::Uuid>,

    pub approved_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<uuid::Uuid>,

    // Report data (JSONB structure)
    #[column_type = "JSONB"]
    pub report_data: Option<Value>, // Generated report data

    // Parameters
    #[column_type = "JSONB"]
    pub parameters: Option<Value>, // Report generation parameters

    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Multi-company
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
