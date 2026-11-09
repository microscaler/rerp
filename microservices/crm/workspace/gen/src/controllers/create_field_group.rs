// User-owned controller for handler 'create_field_group'.

use crate::handlers::create_field_group::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFieldGroupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        entity: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        sequence: Some(42),
    })
}
