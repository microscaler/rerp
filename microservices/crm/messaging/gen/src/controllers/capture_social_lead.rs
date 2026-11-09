// User-owned controller for handler 'capture_social_lead'.

use crate::handlers::capture_social_lead::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CaptureSocialLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        lead_id: Some("example".to_string()),
        matched_existing: Some(true),
    })
}
