//! Report Template entity
//!
//! Templates for generating financial reports.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "report_templates"]
#[skip_from_row]
#[table_comment = "Financial report templates"]
#[index = "idx_report_templates_template_code(template_code)"]
#[index = "idx_report_templates_report_type(report_type)"]
pub struct ReportTemplate {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Template identification
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub template_code: String,
    
    #[column_type = "VARCHAR(255)"]
    pub name: String,
    
    pub description: Option<String>,
    
    // Template type
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub report_type: String, // PROFIT_LOSS, BALANCE_SHEET, CASH_FLOW, etc.
    
    // Template structure (JSONB)
    #[column_type = "JSONB"]
    pub template_structure: Value, // Report structure definition
    
    // Formulas and calculations
    #[column_type = "JSONB"]
    pub formulas: Option<Value>, // Calculation formulas
    
    // Formatting
    #[column_type = "JSONB"]
    pub formatting: Option<Value>, // Formatting rules
    
    // Account mappings
    #[column_type = "JSONB"]
    pub account_mappings: Option<Value>, // Which accounts appear in which sections
    
    // Status
    #[default_value = "true"]
    pub is_active: bool,
    
    pub is_system_template: bool, // System templates cannot be deleted
    
    // Version
    #[default_value = "1"]
    pub version: i32,
    
    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
    
    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
