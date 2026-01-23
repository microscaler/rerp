//! Report Schedule entity
//!
//! Scheduled report generation (daily, monthly, quarterly, etc.).

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "report_schedules"]
#[skip_from_row]
#[table_comment = "Scheduled financial report generation"]
#[index = "idx_report_schedules_template_id(template_id)"]
#[index = "idx_report_schedules_next_run_at(next_run_at)"]
#[index = "idx_report_schedules_status(status)"]
pub struct ReportSchedule {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Template reference
    #[foreign_key = "report_templates(id) ON DELETE CASCADE"]
    #[indexed]
    pub template_id: uuid::Uuid,
    
    // Schedule name
    #[column_type = "VARCHAR(255)"]
    pub name: String,
    
    pub description: Option<String>,
    
    // Schedule frequency
    #[column_type = "VARCHAR(50)"]
    pub frequency: String, // DAILY, WEEKLY, MONTHLY, QUARTERLY, YEARLY, CUSTOM
    
    // Schedule configuration
    #[column_type = "JSONB"]
    pub schedule_config: Option<Value>, // Frequency-specific configuration
    
    // Next run
    #[indexed]
    pub next_run_at: Option<chrono::NaiveDateTime>,
    
    pub last_run_at: Option<chrono::NaiveDateTime>,
    
    // Run count
    #[default_value = "0"]
    pub run_count: i32,
    
    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // ACTIVE, PAUSED, COMPLETED, ERROR
    
    // Recipients
    #[column_type = "JSONB"]
    pub recipients: Option<Value>, // Email addresses, user IDs, etc.
    
    // Report parameters
    #[column_type = "JSONB"]
    pub default_parameters: Option<Value>, // Default parameters for scheduled runs
    
    // Output format
    #[column_type = "VARCHAR(50)"]
    pub output_format: Option<String>, // PDF, EXCEL, CSV, JSON
    
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
