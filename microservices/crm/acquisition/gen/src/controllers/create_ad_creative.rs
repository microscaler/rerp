// User-owned controller for handler 'create_ad_creative'.

use crate::handlers::create_ad_creative::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAdCreativeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "",
    //   "brand_kit_applied": true,
    //   "call_to_action": "LEARN_MORE",
    //   "created_at": "2026-10-30T16:00:00Z",
    //   "duration_seconds": 3,
    //   "headline": "48 hours before everyone else",
    //   "height": 628,
    //   "id": "30f4c733-b7f5-51d2-b900-0be84807ebb2",
    //   "kind": "IMAGE",
    //   "landing_url": "https://acme.example/early-access",
    //   "name": "BF hero",
    //   "platforms": [
    //     {},
    //     {},
    //     {}
    //   ],
    //   "width": 1200
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body": "",
  "brand_kit_applied": true,
  "call_to_action": "LEARN_MORE",
  "created_at": "2026-10-30T16:00:00Z",
  "duration_seconds": 3,
  "headline": "48 hours before everyone else",
  "height": 628,
  "id": "30f4c733-b7f5-51d2-b900-0be84807ebb2",
  "kind": "IMAGE",
  "landing_url": "https://acme.example/early-access",
  "name": "BF hero",
  "platforms": [
    {},
    {},
    {}
  ],
  "width": 1200
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        asset_id: Some("example".to_string()),
        body: Some("".to_string()),
        brand_kit_applied: Some(true),
        call_to_action: Some("LEARN_MORE".to_string()),
        created_at: Some("2026-10-30T16:00:00Z".to_string()),
        duration_seconds: Some(3),
        headline: Some("48 hours before everyone else".to_string()),
        height: Some(628),
        id: "30f4c733-b7f5-51d2-b900-0be84807ebb2".to_string(),
        kind: "IMAGE".to_string(),
        landing_url: Some("https://acme.example/early-access".to_string()),
        name: "BF hero".to_string(),
        platforms: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ]),
        url: Some("example".to_string()),
        width: Some(1200),
    })
}
