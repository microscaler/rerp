// User-owned controller for handler 'register_event'.

use crate::handlers::register_event::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RegisterEventController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        attended: Some(true),
        attendee_company: Some("example".to_string()),
        attendee_email: "example".to_string(),
        attendee_function: Some("example".to_string()),
        attendee_name: Some("example".to_string()),
        attendee_phone: Some("example".to_string()),
        event_date: Some("example".to_string()),
        event_name: "example".to_string(),
        id: "example".to_string(),
        lead_id: Some("example".to_string()),
        registration_date: Some("example".to_string()),
        source: Some("example".to_string()),
    })
}
