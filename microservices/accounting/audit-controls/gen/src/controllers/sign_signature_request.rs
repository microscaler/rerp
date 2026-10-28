// User-owned controller for handler 'sign_signature_request'.

use crate::handlers::sign_signature_request::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SignSignatureRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        entity_id: "example".to_string(),
        id: "example".to_string(),
        requested_by: "example".to_string(),
        service_name: "example".to_string(),
        status: Default::default(),
    })
}
