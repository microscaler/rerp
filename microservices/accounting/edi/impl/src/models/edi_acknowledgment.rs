//! EDI Acknowledgment entity
//!
//! EDI acknowledgments (997, CONTRL, etc.) sent and received.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "edi_acknowledgments"]
#[skip_from_row]
#[table_comment = "EDI acknowledgments"]
#[index = "idx_edi_acknowledgments_document_id(document_id)"]
#[index = "idx_edi_acknowledgments_acknowledgment_type(acknowledgment_type)"]
pub struct EdiAcknowledgment {
    #[primary_key]
    pub id: uuid::Uuid,

    // Document reference
    #[foreign_key = "edi_documents(id) ON DELETE CASCADE"]
    #[indexed]
    pub document_id: uuid::Uuid,

    // Acknowledgment details
    #[column_type = "VARCHAR(50)"]
    pub acknowledgment_type: String, // FUNCTIONAL, TECHNICAL, RECEIPT

    #[column_type = "VARCHAR(50)"]
    pub status: String, // ACCEPTED, REJECTED, PARTIAL

    // Acknowledgment content
    #[column_type = "TEXT"]
    pub acknowledgment_content: Option<String>, // Raw acknowledgment

    #[column_type = "JSONB"]
    pub acknowledgment_data: Option<Value>, // Parsed acknowledgment data

    // Processing
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub received_at: Option<chrono::NaiveDateTime>,

    // Error information (if rejected)
    pub error_codes: Option<Value>, // JSONB array of error codes
    pub error_messages: Option<String>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
