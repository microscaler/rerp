// User-owned controller for handler 'create_segment'.

use crate::handlers::create_segment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConditionGroup;

#[handler(CreateSegmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "conditions": {
    //     "items": [
    //       {
    //         "operator": "HAS",
    //         "subject": "engagement:any:30"
    //       }
    //     ],
    //     "op": "AND"
    //   },
    //   "contact_count": 6204,
    //   "created_at": "2026-08-07T16:00:00Z",
    //   "description": "Opened or clicked in the last 30 days",
    //   "id": "20df9eb1-f769-5766-b664-2f6b6529022c",
    //   "name": "Engaged · 30 days"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "conditions": {
    "items": [
      {
        "operator": "HAS",
        "subject": "engagement:any:30"
      }
    ],
    "op": "AND"
  },
  "contact_count": 6204,
  "created_at": "2026-08-07T16:00:00Z",
  "description": "Opened or clicked in the last 30 days",
  "id": "20df9eb1-f769-5766-b664-2f6b6529022c",
  "name": "Engaged · 30 days"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        conditions: serde_json::from_value::<ConditionGroup>(serde_json::json!({"items":[{"operator":"HAS","subject":"engagement:any:30"}],"op":"AND"})).unwrap_or_default(),contact_count: Some(6204),created_at: "2026-08-07T16:00:00Z".to_string(),description: Some("Opened or clicked in the last 30 days".to_string()),id: "20df9eb1-f769-5766-b664-2f6b6529022c".to_string(),name: "Engaged · 30 days".to_string(),updated_at: Some("example".to_string()),
    })
}
