//! EDI Document entity
//!
//! EDI documents (invoices, purchase orders, etc.) processed through the system.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "edi_documents"]
#[skip_from_row]
#[table_comment = "EDI documents"]
#[index = "idx_edi_documents_document_number(document_number)"]
#[index = "idx_edi_documents_document_type(document_type)"]
#[index = "idx_edi_documents_status(status)"]
#[index = "idx_edi_documents_format(format_id)"]
pub struct EdiDocument {
    #[primary_key]
    pub id: uuid::Uuid,

    // Document identification
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub document_number: String,

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub document_type: String, // INVOICE, PURCHASE_ORDER, ACKNOWLEDGMENT, etc.

    // EDI format
    #[foreign_key = "edi_formats(id) ON DELETE RESTRICT"]
    #[indexed]
    pub format_id: uuid::Uuid,

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // RECEIVED, PARSING, PARSED, VALIDATED, PROCESSED, ERROR

    // Processing dates
    pub received_at: chrono::NaiveDateTime,
    pub parsed_at: Option<chrono::NaiveDateTime>,
    pub validated_at: Option<chrono::NaiveDateTime>,
    pub processed_at: Option<chrono::NaiveDateTime>,

    // Source/Destination
    #[column_type = "VARCHAR(255)"]
    pub sender_id: Option<String>, // Trading partner ID

    #[column_type = "VARCHAR(255)"]
    pub receiver_id: Option<String>, // Our trading partner ID

    // Raw and processed data
    #[column_type = "TEXT"]
    pub raw_content: Option<String>, // Original EDI content

    #[column_type = "JSONB"]
    pub parsed_data: Option<Value>, // Parsed EDI data as JSON

    // Related records
    pub related_invoice_id: Option<uuid::Uuid>, // If this EDI created/updated an invoice
    pub related_purchase_order_id: Option<uuid::Uuid>,

    // Error handling
    pub error_message: Option<String>,
    pub error_details: Option<Value>, // JSONB error details

    pub retry_count: i32,
    pub last_retry_at: Option<chrono::NaiveDateTime>,

    // Acknowledgment
    pub acknowledgment_sent: bool,
    pub acknowledgment_sent_at: Option<chrono::NaiveDateTime>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,

    // Multi-company
    pub company_id: Option<uuid::Uuid>,

    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
