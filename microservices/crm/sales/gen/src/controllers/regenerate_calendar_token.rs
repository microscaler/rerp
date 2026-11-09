// User-owned controller for handler 'regenerate_calendar_token'.

use crate::handlers::regenerate_calendar_token::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RegenerateCalendarTokenController)]
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
