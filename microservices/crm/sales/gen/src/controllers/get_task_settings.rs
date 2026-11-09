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
    //   "allow_unlinked_tasks": false,
    //   "default_reminder_minutes": 15,
    //   "require_outcomes": true,
    //   "updated_at": "2026-11-03T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "allow_unlinked_tasks": false,
  "default_reminder_minutes": 15,
  "require_outcomes": true,
  "updated_at": "2026-11-03T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        allow_unlinked_tasks: Some(false),
        default_due_offset_minutes: Some(42),
        default_reminder_minutes: Some(15),
        default_reminders: Some(vec![]),
        require_outcomes: Some(true),
        updated_at: Some("2026-11-03T16:00:00Z".to_string()),
    })
}
