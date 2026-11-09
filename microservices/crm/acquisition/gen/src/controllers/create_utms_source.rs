// User-owned controller for handler 'create_utms_source'.

use crate::handlers::create_utms_source::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateUtmsSourceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: Some(true),
        id: "example".to_string(),
        medium_id: Some("example".to_string()),
        name: "example".to_string(),
    })
}
