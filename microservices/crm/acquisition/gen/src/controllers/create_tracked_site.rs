// User-owned controller for handler 'create_tracked_site'.

use crate::handlers::create_tracked_site::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTrackedSiteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: "example".to_string(),
        enabled: Some(true),
        id: "example".to_string(),
        include_all_pages: Some(true),
        last_event_at: Some("example".to_string()),
        name: Some("example".to_string()),
        origin: "example".to_string(),
        snippet: Some("example".to_string()),
        tracking_key: "example".to_string(),
    })
}
