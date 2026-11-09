// User-owned controller for handler 'reorder_type'.

use crate::handlers::reorder_type::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReorderTypeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: true,
        created_at: Some("example".to_string()),
        default_minutes: Some(42),
        id: "example".to_string(),
        open_count: Some(42),
        outcome_ids: Some(vec![]),
        outcomes: Some(vec![]),
        sequence: Some(42),
        title: "example".to_string(),
    })
}
