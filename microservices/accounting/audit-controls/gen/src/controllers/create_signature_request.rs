// User-owned controller for handler 'create_signature_request'.

use crate::handlers::create_signature_request::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateSignatureRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        entity_id: "example".to_string(),
        id: "example".to_string(),
        requested_by: "example".to_string(),
        service_name: "example".to_string(),
        status: Default::default(),
    }
}
