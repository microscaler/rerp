// User-owned controller for handler 'update_attribution_model'.

use crate::handlers::update_attribution_model::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAttributionModelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "builtin": true,
    //   "count_kinds": [
    //     "AD_CLICK",
    //     "EMAIL_CLICKED",
    //     "SITE_VISIT"
    //   ],
    //   "id": "last_touch",
    //   "is_default": true,
    //   "kind": "LAST_TOUCH",
    //   "name": "Last touch",
    //   "weights": {
    //     "assists": 0,
    //     "first": 0,
    //     "last": 1
    //   },
    //   "window_days": 30
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "builtin": true,
  "count_kinds": [
    "AD_CLICK",
    "EMAIL_CLICKED",
    "SITE_VISIT"
  ],
  "id": "last_touch",
  "is_default": true,
  "kind": "LAST_TOUCH",
  "name": "Last touch",
  "weights": {
    "assists": 0,
    "first": 0,
    "last": 1
  },
  "window_days": 30
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        builtin: Some(true),
        count_kinds: Some(vec![
            "AD_CLICK".to_string(),
            "EMAIL_CLICKED".to_string(),
            "SITE_VISIT".to_string(),
        ]),
        half_life_days: Some(1.5),
        id: Some("last_touch".to_string()),
        is_default: Some(true),
        kind: Some("LAST_TOUCH".to_string()),
        name: "Last touch".to_string(),
        weights: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"assists":0,"first":0,"last":1}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        window_days: Some(30),
    })
}
