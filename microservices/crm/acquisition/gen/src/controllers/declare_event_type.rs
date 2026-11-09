// User-owned controller for handler 'declare_event_type'.

use crate::handlers::declare_event_type::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeclareEventTypeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count_30d": 2140,
    //   "declared": true,
    //   "description": "First successful import in the product",
    //   "last_seen_at": "2026-11-05T15:58:00Z",
    //   "name": "activated"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "count_30d": 2140,
  "declared": true,
  "description": "First successful import in the product",
  "last_seen_at": "2026-11-05T15:58:00Z",
  "name": "activated"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        count_30d: Some(2140),
        declared: Some(true),
        description: Some("First successful import in the product".to_string()),
        enabled: Some(true),
        last_seen_at: Some("2026-11-05T15:58:00Z".to_string()),
        name: "activated".to_string(),
    })
}
