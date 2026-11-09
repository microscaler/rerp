// User-owned controller for handler 'create_suppression'.

use crate::handlers::create_suppression::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::SuppressionReason;

#[handler(CreateSuppressionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "address": "daniel.kim@wideworld.example",
    //   "channel": "EMAIL",
    //   "created_at": "2026-10-28T16:00:00Z",
    //   "id": "fc58b14d-e13f-5d43-bad7-0a86f5d330c6",
    //   "note": "October product update",
    //   "reason": "HARD_BOUNCE"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "address": "daniel.kim@wideworld.example",
  "channel": "EMAIL",
  "created_at": "2026-10-28T16:00:00Z",
  "id": "fc58b14d-e13f-5d43-bad7-0a86f5d330c6",
  "note": "October product update",
  "reason": "HARD_BOUNCE"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        address: "daniel.kim@wideworld.example".to_string(),
        channel: Some(
            serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string()))
                .unwrap_or_default(),
        ),
        created_at: "2026-10-28T16:00:00Z".to_string(),
        id: "fc58b14d-e13f-5d43-bad7-0a86f5d330c6".to_string(),
        note: Some("October product update".to_string()),
        reason: serde_json::from_value::<SuppressionReason>(serde_json::Value::String(
            "HARD_BOUNCE".to_string(),
        ))
        .unwrap_or_default(),
        source_send_id: Some("example".to_string()),
    })
}
