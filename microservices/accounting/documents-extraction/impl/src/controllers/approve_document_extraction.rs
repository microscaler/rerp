// Implementation stub for handler 'approve_document_extraction'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::approve_document_extraction::{Request, Response};

#[handler(ApproveDocumentExtractionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        approved_at: "".to_string(),
        approved_by: "".to_string(),
        document_id: "".to_string(),
    }
}
