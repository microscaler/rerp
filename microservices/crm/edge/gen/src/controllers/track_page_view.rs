// User-owned controller for handler 'track_page_view'.

use crate::handlers::track_page_view::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TrackPageViewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        city: Some("example".to_string()),
        converted_to_lead: Some(true),
        country: Some("example".to_string()),
        first_seen: "example".to_string(),
        id: "example".to_string(),
        last_seen: Some("example".to_string()),
        lead_id: Some("example".to_string()),
        pages_viewed: Some(42),
        referrer: Some("example".to_string()),
        url_visited: Some("example".to_string()),
        user_agent: Some("example".to_string()),
        visitor_id: "example".to_string(),
    })
}
