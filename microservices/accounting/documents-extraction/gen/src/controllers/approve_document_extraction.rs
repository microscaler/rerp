// User-owned controller for handler 'approve_document_extraction'.

use crate::handlers::approve_document_extraction::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ApproveDocumentExtractionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        approved_at: "example".to_string(),
        approved_by: "example".to_string(),
        document_id: "example".to_string(),
    }
}
