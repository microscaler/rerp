// User-owned controller for handler 'create_list_exclusion'.

use crate::handlers::create_list_exclusion::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateListExclusionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        id: "example".to_string(),
        list_ids: Some(vec![]),
        list_names: Some(vec![]),
        pattern: "example".to_string(),
        reason: Some("example".to_string()),
    })
}
