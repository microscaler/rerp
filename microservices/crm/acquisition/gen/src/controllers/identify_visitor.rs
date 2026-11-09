// User-owned controller for handler 'identify_visitor'.

use crate::handlers::identify_visitor::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(IdentifyVisitorController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        identified: Some(true),
        lead_id: Some("example".to_string()),
        visitor_id: Some("example".to_string()),
    })
}
