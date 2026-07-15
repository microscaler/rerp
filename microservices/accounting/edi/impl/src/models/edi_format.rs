//! EDI Format entity
//!
//! EDI format definitions (EDIFACT, X12, etc.).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "edi_formats"]
#[skip_from_row]
#[table_comment = "EDI format definitions"]
#[index = "idx_edi_formats_code(code)"]
pub struct EdiFormat {
    #[primary_key]
    pub id: uuid::Uuid,

    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub code: String, // EDIFACT, X12_810, X12_850, etc.

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    pub description: Option<String>,

    // Format version
    #[column_type = "VARCHAR(50)"]
    pub version: Option<String>, // e.g., "D.96A" for EDIFACT

    // Document types supported
    #[column_type = "VARCHAR(255)"]
    pub supported_document_types: Option<String>, // Comma-separated list

    // Configuration
    #[column_type = "JSONB"]
    pub format_config: Option<serde_json::Value>, // Format-specific configuration

    #[default_value = "true"]
    pub is_active: bool,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
