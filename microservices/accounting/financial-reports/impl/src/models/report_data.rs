//! Report Data entity
//!
//! Generated report data snapshots.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "report_data"]
#[skip_from_row]
#[table_comment = "Generated financial report data"]
#[index = "idx_report_data_report_id(report_id)"]
#[index = "idx_report_data_report_date(report_date)"]
pub struct ReportData {
    #[primary_key]
    pub id: uuid::Uuid,

    // Report reference
    #[foreign_key = "financial_reports(id) ON DELETE CASCADE"]
    #[indexed]
    pub report_id: uuid::Uuid,

    // Data snapshot date
    #[indexed]
    pub report_date: chrono::NaiveDate,

    // Report data (JSONB structure)
    #[column_type = "JSONB"]
    pub data: Value, // Actual report data

    // Summary totals
    #[column_type = "JSONB"]
    pub summary: Option<Value>, // Summary totals and key metrics

    // Generation metadata
    pub generated_at: chrono::NaiveDateTime,
    pub generated_by: Option<uuid::Uuid>,

    // Data version
    #[default_value = "1"]
    pub data_version: i32,

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
}
