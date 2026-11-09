// User-owned controller for handler 'get_pipeline_summary'.

use crate::handlers::get_pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "open": 3,
    //   "open_value": "1250.00",
    //   "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
    //   "stages": [
    //     {}
    //   ],
    //   "stalled": 3,
    //   "stalled_value": "1250.00",
    //   "weighted_value": "1250.00",
    //   "won_this_month": 3,
    //   "won_value_this_month": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "open": 3,
  "open_value": "1250.00",
  "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
  "stages": [
    {}
  ],
  "stalled": 3,
  "stalled_value": "1250.00",
  "weighted_value": "1250.00",
  "won_this_month": 3,
  "won_value_this_month": "1250.00"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        open: Some(3),
        open_value: Some("1250.00".to_string()),
        pipeline_id: Some("32dd9c82-6ef8-54b5-816b-7f32b5de5050".to_string()),
        stages: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        stalled: Some(3),
        stalled_value: Some("1250.00".to_string()),
        weighted_value: Some("1250.00".to_string()),
        won_this_month: Some(3),
        won_value_this_month: Some("1250.00".to_string()),
    })
}
