// User-owned controller for handler 'declare_event_type'.

use crate::handlers::declare_event_type::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeclareEventTypeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        count_30d: Some(42),
        declared: Some(true),
        description: Some("example".to_string()),
        enabled: Some(true),
        last_seen_at: Some("example".to_string()),
        name: "example".to_string(),
    })
}
