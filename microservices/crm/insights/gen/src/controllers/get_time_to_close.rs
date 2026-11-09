// User-owned controller for handler 'get_time_to_close'.

use crate::handlers::get_time_to_close::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTimeToCloseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_source": [
    //     {}
    //   ],
    //   "by_stage": [
    //     {}
    //   ],
    //   "overall_avg_days": 12.5
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "by_source": [
    {}
  ],
  "by_stage": [
    {}
  ],
  "overall_avg_days": 12.5
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_source: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        by_stage: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        overall_avg_days: 12.5,
    })
}
