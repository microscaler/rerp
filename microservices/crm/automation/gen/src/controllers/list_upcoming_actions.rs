// User-owned controller for handler 'list_upcoming_actions'.

use crate::handlers::list_upcoming_actions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ScheduledAction;

#[handler(ListUpcomingActionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //       "scheduled_at": "2026-11-05T16:00:00Z",
    //       "status": "PENDING"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
      "scheduled_at": "2026-11-05T16:00:00Z",
      "status": "PENDING"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ScheduledAction>(serde_json::json!({"id":"c9e18116-9fea-5e6e-ac32-52580d2450b8","scheduled_at":"2026-11-05T16:00:00Z","status":"PENDING"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
