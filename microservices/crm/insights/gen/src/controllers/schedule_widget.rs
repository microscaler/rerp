// User-owned controller for handler 'schedule_widget'.

use crate::handlers::schedule_widget::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ScheduleWidgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        frequency: "example".to_string(),
        id: Some("example".to_string()),
        is_active: true,
        last_sent: "example".to_string(),
        recipient_ids: vec![],
        report_id: "example".to_string(),
    })
}
