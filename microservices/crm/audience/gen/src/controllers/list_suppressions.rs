// User-owned controller for handler 'list_suppressions'.

use crate::handlers::list_suppressions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Suppression;

#[handler(ListSuppressionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "address": "daniel.kim@wideworld.example",
    //       "channel": "EMAIL",
    //       "created_at": "2026-10-28T16:00:00Z",
    //       "id": "fc58b14d-e13f-5d43-bad7-0a86f5d330c6",
    //       "note": "October product update",
    //       "reason": "HARD_BOUNCE"
    //     },
    //     {
    //       "address": "old-team@fabrikam.example",
    //       "channel": "EMAIL",
    //       "created_at": "2026-09-30T16:00:00Z",
    //       "id": "a6c4c319-1fb1-5d3c-b7bc-4626e853c8f7",
    //       "note": "September product update",
    //       "reason": "COMPLAINT"
    //     },
    //     {
    //       "address": "ceo@contoso.example",
    //       "channel": "EMAIL",
    //       "created_at": "2026-10-12T16:00:00Z",
    //       "id": "434e2868-e253-5882-bd99-f063c5c7a6aa",
    //       "note": "asked by account manager",
    //       "reason": "MANUAL"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "address": "daniel.kim@wideworld.example",
      "channel": "EMAIL",
      "created_at": "2026-10-28T16:00:00Z",
      "id": "fc58b14d-e13f-5d43-bad7-0a86f5d330c6",
      "note": "October product update",
      "reason": "HARD_BOUNCE"
    },
    {
      "address": "old-team@fabrikam.example",
      "channel": "EMAIL",
      "created_at": "2026-09-30T16:00:00Z",
      "id": "a6c4c319-1fb1-5d3c-b7bc-4626e853c8f7",
      "note": "September product update",
      "reason": "COMPLAINT"
    },
    {
      "address": "ceo@contoso.example",
      "channel": "EMAIL",
      "created_at": "2026-10-12T16:00:00Z",
      "id": "434e2868-e253-5882-bd99-f063c5c7a6aa",
      "note": "asked by account manager",
      "reason": "MANUAL"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Suppression>(serde_json::json!({"address":"daniel.kim@wideworld.example","channel":"EMAIL","created_at":"2026-10-28T16:00:00Z","id":"fc58b14d-e13f-5d43-bad7-0a86f5d330c6","note":"October product update","reason":"HARD_BOUNCE"})).unwrap_or_default(), serde_json::from_value::<Suppression>(serde_json::json!({"address":"old-team@fabrikam.example","channel":"EMAIL","created_at":"2026-09-30T16:00:00Z","id":"a6c4c319-1fb1-5d3c-b7bc-4626e853c8f7","note":"September product update","reason":"COMPLAINT"})).unwrap_or_default(), serde_json::from_value::<Suppression>(serde_json::json!({"address":"ceo@contoso.example","channel":"EMAIL","created_at":"2026-10-12T16:00:00Z","id":"434e2868-e253-5882-bd99-f063c5c7a6aa","note":"asked by account manager","reason":"MANUAL"})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
