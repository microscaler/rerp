// User-owned controller for handler 'create_extraction_job'.

use crate::handlers::create_extraction_job::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateExtractionJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        completed_at: Some("example".to_string()),
        created_at: "example".to_string(),
        document_id: "example".to_string(),
        id: "example".to_string(),
        profile: Some("example".to_string()),
        status: Default::default(),
    }
}
