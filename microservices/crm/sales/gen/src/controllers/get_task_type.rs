// User-owned controller for handler 'get_task_type'.

use crate::handlers::get_task_type::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTaskTypeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": true,
    //   "default_minutes": 15,
    //   "id": "4eec3526-5869-5e76-820f-ac2358c6b899",
    //   "open_count": 12,
    //   "outcomes": [
    //     "Connected",
    //     "No answer",
    //     "Left voicemail"
    //   ],
    //   "title": "Call"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "active": true,
  "default_minutes": 15,
  "id": "4eec3526-5869-5e76-820f-ac2358c6b899",
  "open_count": 12,
  "outcomes": [
    "Connected",
    "No answer",
    "Left voicemail"
  ],
  "title": "Call"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: true,
        created_at: Some("example".to_string()),
        default_minutes: Some(15),
        id: "4eec3526-5869-5e76-820f-ac2358c6b899".to_string(),
        open_count: Some(12),
        outcome_ids: Some(vec![]),
        outcomes: Some(vec![
            "Connected".to_string(),
            "No answer".to_string(),
            "Left voicemail".to_string(),
        ]),
        sequence: Some(42),
        title: "Call".to_string(),
    })
}
