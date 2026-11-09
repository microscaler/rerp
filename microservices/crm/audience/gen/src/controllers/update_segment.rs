// User-owned controller for handler 'update_segment'.

use crate::handlers::update_segment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConditionGroup;

#[handler(UpdateSegmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        conditions: Default::default(),
        contact_count: Some(42),
        created_at: "example".to_string(),
        description: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
