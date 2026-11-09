// User-owned controller for handler 'create_calendar_feed'.

use crate::handlers::create_calendar_feed::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCalendarFeedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        assignee_ids: Some(vec![]),
        created_at: "example".to_string(),
        created_by: Some("example".to_string()),
        id: "example".to_string(),
        include_alerts: Some(true),
        name: "example".to_string(),
        pipeline_ids: Some(vec![]),
        scope: "example".to_string(),
        token: Some("example".to_string()),
        url: Some("example".to_string()),
    })
}
