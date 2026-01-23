//! EDI Mapping entity
//!
//! Field mappings between EDI formats and internal data structures.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "edi_mappings"]
#[skip_from_row]
#[table_comment = "EDI field mappings"]
#[index = "idx_edi_mappings_format_id(format_id)"]
#[index = "idx_edi_mappings_document_type(document_type)"]
pub struct EdiMapping {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Format and document type
    #[foreign_key = "edi_formats(id) ON DELETE CASCADE"]
    #[indexed]
    pub format_id: uuid::Uuid,
    
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub document_type: String, // INVOICE, PURCHASE_ORDER, etc.
    
    // Mapping configuration
    #[column_type = "VARCHAR(100)"]
    pub mapping_name: String, // e.g., "Invoice to CustomerInvoice"
    
    // Field mappings (JSONB structure)
    #[column_type = "JSONB"]
    pub field_mappings: Value, // Maps EDI fields to internal fields
    
    // Transformation rules
    #[column_type = "JSONB"]
    pub transformation_rules: Option<Value>, // Data transformation rules
    
    // Validation rules
    #[column_type = "JSONB"]
    pub validation_rules: Option<Value>, // Field validation rules
    
    // Default values
    #[column_type = "JSONB"]
    pub default_values: Option<Value>, // Default values for fields
    
    // Status
    #[default_value = "true"]
    pub is_active: bool,
    
    pub description: Option<String>,
    
    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
    
    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
