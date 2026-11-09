// User-owned controller for handler 'list_ad_creatives'.

use crate::handlers::list_ad_creatives::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdCreative;

#[handler(ListAdCreativesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "body": "",
    //       "brand_kit_applied": true,
    //       "call_to_action": "LEARN_MORE",
    //       "created_at": "2026-10-30T16:00:00Z",
    //       "duration_seconds": 3,
    //       "headline": "48 hours before everyone else",
    //       "height": 628,
    //       "id": "30f4c733-b7f5-51d2-b900-0be84807ebb2",
    //       "kind": "IMAGE",
    //       "landing_url": "https://acme.example/early-access",
    //       "name": "BF hero",
    //       "platforms": [
    //         {},
    //         {},
    //         {}
    //       ],
    //       "width": 1200
    //     },
    //     {
    //       "body": "",
    //       "brand_kit_applied": true,
    //       "call_to_action": "LEARN_MORE",
    //       "created_at": "2026-10-31T16:00:00Z",
    //       "duration_seconds": 3,
    //       "headline": "Early access · 21 Nov",
    //       "height": 1080,
    //       "id": "8767be08-2adf-5232-a68d-eefde450d501",
    //       "kind": "IMAGE",
    //       "landing_url": "https://acme.example/early-access",
    //       "name": "BF square",
    //       "platforms": [
    //         {},
    //         {}
    //       ],
    //       "width": 1080
    //     },
    //     {
    //       "body": "",
    //       "brand_kit_applied": true,
    //       "call_to_action": "LEARN_MORE",
    //       "created_at": "2026-11-01T16:00:00Z",
    //       "duration_seconds": 15,
    //       "headline": "Pricing that holds up",
    //       "height": 1080,
    //       "id": "3998574b-7118-511a-bd3b-fad92e7cddfd",
    //       "kind": "VIDEO",
    //       "landing_url": "https://acme.example/webinar",
    //       "name": "Webinar",
    //       "platforms": [
    //         {}
    //       ],
    //       "width": 1920
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
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
    },
    {
      "body": "",
      "brand_kit_applied": true,
      "call_to_action": "LEARN_MORE",
      "created_at": "2026-10-31T16:00:00Z",
      "duration_seconds": 3,
      "headline": "Early access · 21 Nov",
      "height": 1080,
      "id": "8767be08-2adf-5232-a68d-eefde450d501",
      "kind": "IMAGE",
      "landing_url": "https://acme.example/early-access",
      "name": "BF square",
      "platforms": [
        {},
        {}
      ],
      "width": 1080
    },
    {
      "body": "",
      "brand_kit_applied": true,
      "call_to_action": "LEARN_MORE",
      "created_at": "2026-11-01T16:00:00Z",
      "duration_seconds": 15,
      "headline": "Pricing that holds up",
      "height": 1080,
      "id": "3998574b-7118-511a-bd3b-fad92e7cddfd",
      "kind": "VIDEO",
      "landing_url": "https://acme.example/webinar",
      "name": "Webinar",
      "platforms": [
        {}
      ],
      "width": 1920
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-10-30T16:00:00Z","duration_seconds":3,"headline":"48 hours before everyone else","height":628,"id":"30f4c733-b7f5-51d2-b900-0be84807ebb2","kind":"IMAGE","landing_url":"https://acme.example/early-access","name":"BF hero","platforms":[{},{},{}],"width":1200})).unwrap_or_default(), serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-10-31T16:00:00Z","duration_seconds":3,"headline":"Early access · 21 Nov","height":1080,"id":"8767be08-2adf-5232-a68d-eefde450d501","kind":"IMAGE","landing_url":"https://acme.example/early-access","name":"BF square","platforms":[{},{}],"width":1080})).unwrap_or_default(), serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-11-01T16:00:00Z","duration_seconds":15,"headline":"Pricing that holds up","height":1080,"id":"3998574b-7118-511a-bd3b-fad92e7cddfd","kind":"VIDEO","landing_url":"https://acme.example/webinar","name":"Webinar","platforms":[{}],"width":1920})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
