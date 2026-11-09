// User-owned controller for handler 'get_pipeline_summary'.

use crate::handlers::get_pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "stages": [
    //     {}
    //   ],
    //   "total_opportunities": 3,
    //   "total_revenue": "1250.00",
    //   "total_weighted_revenue": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "stages": [
    {}
  ],
  "total_opportunities": 3,
  "total_revenue": "1250.00",
  "total_weighted_revenue": "1250.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        stages: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        total_opportunities: 3,
        total_revenue: "1250.00".to_string(),
        total_weighted_revenue: "1250.00".to_string(),
    })
}
