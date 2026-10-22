// User-owned controller for handler 'convert_lead'.

use crate::handlers::convert_lead::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ConvertLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        conversion_id: "example".to_string(),
        lead_id: "example".to_string(),
        message_id: Some("example".to_string()),
        reason: "example".to_string(),
        stage: "example".to_string(),
    })
}
