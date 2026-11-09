// User-owned controller for handler 'create_tag'.

use crate::handlers::create_tag::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTagController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        color: Some("example".to_string()),
        contact_count: Some(42),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
    })
}
