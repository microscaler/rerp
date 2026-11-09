// User-owned controller for handler 'get_funnel'.

use crate::handlers::get_funnel::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetFunnelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "avg_days_to_close": 34,
    //   "items": [
    //     {
    //       "count": 122,
    //       "label": "Created"
    //     },
    //     {
    //       "count": 84,
    //       "label": "Qualified"
    //     },
    //     {
    //       "count": 51,
    //       "label": "Proposal"
    //     }
    //   ],
    //   "stages": [
    //     {}
    //   ],
    //   "win_rate": 0.31
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "avg_days_to_close": 34,
  "items": [
    {
      "count": 122,
      "label": "Created"
    },
    {
      "count": 84,
      "label": "Qualified"
    },
    {
      "count": 51,
      "label": "Proposal"
    }
  ],
  "stages": [
    {}
  ],
  "win_rate": 0.31
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        avg_days_to_close: Some(34),
        items: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{"count":122,"label":"Created"}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"count":84,"label":"Qualified"}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"count":51,"label":"Proposal"}"#)
                .unwrap_or(serde_json::Value::Null),
        ]),
        stages: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        win_rate: Some(0.31),
    })
}
