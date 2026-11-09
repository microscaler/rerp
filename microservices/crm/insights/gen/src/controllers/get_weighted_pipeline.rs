// User-owned controller for handler 'get_weighted_pipeline'.

use crate::handlers::get_weighted_pipeline::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWeightedPipelineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_team": [
    //     {}
    //   ],
    //   "by_user": [
    //     {}
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "by_team": [
    {}
  ],
  "by_user": [
    {}
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_team: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        by_user: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
    })
}
