// User-owned controller for handler 'create_utms_medium'.

use crate::handlers::create_utms_medium::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateUtmsMediumController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: Some(true),
        id: "example".to_string(),
        name: "example".to_string(),
    })
}
