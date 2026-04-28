// Implementation stub for handler 'create_extraction_job'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::create_extraction_job::{Request, Response};

#[handler(CreateExtractionJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        completed_at: None,
        created_at: "".to_string(),
        document_id: "".to_string(),
        id: "".to_string(),
        profile: None,
        status: serde_json::json!({}),
    }
}
