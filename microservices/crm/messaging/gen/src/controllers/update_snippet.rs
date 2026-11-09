// User-owned controller for handler 'update_snippet'.

use crate::handlers::update_snippet::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateSnippetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        key: "example".to_string(),
        updated_at: Some("example".to_string()),
        used_by: Some(42),
        value: "example".to_string(),
    })
}
