// User-owned controller for handler 'update_edi_document'.
use crate::handlers::update_edi_document::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateEdiDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        acknowledgment_sent: true,
        acknowledgment_sent_at: Some("example".to_string()),
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        document_number: "example".to_string(),
        document_type: "example".to_string(),
        error_details: Some(Default::default()),
        error_message: Some("example".to_string()),
        format_id: "example".to_string(),
        id: "example".to_string(),
        last_retry_at: Some("example".to_string()),
        metadata: Some(Default::default()),
        parsed_at: Some("example".to_string()),
        parsed_data: Some(Default::default()),
        processed_at: Some("example".to_string()),
        raw_content: Some("example".to_string()),
        received_at: "example".to_string(),
        receiver_id: Some("example".to_string()),
        related_invoice_id: Some("example".to_string()),
        related_purchase_order_id: Some("example".to_string()),
        retry_count: 42,
        sender_id: Some("example".to_string()),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
        validated_at: Some("example".to_string()),
    }
}
