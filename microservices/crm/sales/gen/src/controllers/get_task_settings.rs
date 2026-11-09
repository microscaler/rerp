// User-owned controller for handler 'get_task_settings'.

use crate::handlers::get_task_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Reminder;

#[handler(GetTaskSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "require_outcomes": true
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "require_outcomes": true
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        default_due_offset_minutes: Some(42),
        default_reminders: Some(vec![]),
        require_outcomes: Some(true),
    })
}
