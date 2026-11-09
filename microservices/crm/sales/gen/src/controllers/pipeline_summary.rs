// User-owned controller for handler 'pipeline_summary'.

use crate::handlers::pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "stages": [
    //     {
    //       "count": 9,
    //       "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "stage_name": "New",
    //       "weighted_revenue": "4100.00"
    //     },
    //     {
    //       "count": 8,
    //       "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
    //       "stage_name": "Qualified",
    //       "weighted_revenue": "24100.00"
    //     },
    //     {
    //       "count": 7,
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "weighted_revenue": "59250.00"
    //     }
    //   ],
    //   "total_opportunities": 3,
    //   "total_revenue": "1250.00",
    //   "total_weighted_revenue": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "stages": [
    {
      "count": 9,
      "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "stage_name": "New",
      "weighted_revenue": "4100.00"
    },
    {
      "count": 8,
      "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
      "stage_name": "Qualified",
      "weighted_revenue": "24100.00"
    },
    {
      "count": 7,
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "stage_name": "Proposal",
      "weighted_revenue": "59250.00"
    }
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
        stages: vec![serde_json::from_str::<serde_json::Value>(r#"{"count":9,"stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","weighted_revenue":"4100.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":8,"stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","stage_name":"Qualified","weighted_revenue":"24100.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":7,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","weighted_revenue":"59250.00"}"#).unwrap_or(serde_json::Value::Null)],total_opportunities: 3,total_revenue: "1250.00".to_string(),total_weighted_revenue: "1250.00".to_string(),
    })
}
