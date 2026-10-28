// User-owned controller for handler 'create_control_exception'.

use crate::handlers::create_control_exception::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateControlExceptionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        entity_id: "example".to_string(),
        id: "example".to_string(),
        reason: "example".to_string(),
        service_name: "example".to_string(),
        status: Default::default(),
    })
}
