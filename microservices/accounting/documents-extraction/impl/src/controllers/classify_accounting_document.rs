// Implementation stub for handler 'classify_accounting_document'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::classify_accounting_document::{Request, Response};

#[handler(ClassifyAccountingDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        confidence: 0.0,
        document_id: "".to_string(),
        document_type: serde_json::json!({}),
        reasons: None,
    }
}
