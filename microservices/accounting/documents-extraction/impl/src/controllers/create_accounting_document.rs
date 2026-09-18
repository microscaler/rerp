// Implementation stub for handler 'create_accounting_document'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::create_accounting_document::{Request, Response};

#[handler(CreateAccountingDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        company_id: None,
        content_type: None,
        document_type: serde_json::json!({}),
        file_name: "".to_string(),
        id: "".to_string(),
        source_uri: None,
        status: serde_json::json!({}),
        uploaded_at: "".to_string(),
    }
}
