// User-owned controller for handler 'update_task_settings'.

use crate::handlers::update_task_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Reminder;

#[handler(UpdateTaskSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        default_due_offset_minutes: Some(42),
        default_reminders: Some(vec![]),
        require_outcomes: Some(true),
    })
}
