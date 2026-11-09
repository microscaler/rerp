// User-owned controller for handler 'pipeline_summary'.

use crate::handlers::pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "open": 38,
    //   "open_value": 412300,
    //   "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //   "stages": [
    //     {
    //       "count": 9,
    //       "probability": 10,
    //       "revenue": 41000,
    //       "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "stage_name": "New",
    //       "weighted_revenue": "4100.00"
    //     },
    //     {
    //       "count": 8,
    //       "probability": 25,
    //       "revenue": 96400,
    //       "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
    //       "stage_name": "Qualified",
    //       "weighted_revenue": "24100.00"
    //     },
    //     {
    //       "count": 7,
    //       "probability": 50,
    //       "revenue": 118500,
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "weighted_revenue": "59250.00"
    //     }
    //   ],
    //   "stalled": 6,
    //   "stalled_value": 58200,
    //   "total_opportunities": 3,
    //   "total_revenue": "1250.00",
    //   "total_weighted_revenue": "1250.00",
    //   "weighted_value": 188900,
    //   "won_this_month": 7,
    //   "won_value_this_month": 55200
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "open": 38,
  "open_value": 412300,
  "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
  "stages": [
    {
      "count": 9,
      "probability": 10,
      "revenue": 41000,
      "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "stage_name": "New",
      "weighted_revenue": "4100.00"
    },
    {
      "count": 8,
      "probability": 25,
      "revenue": 96400,
      "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
      "stage_name": "Qualified",
      "weighted_revenue": "24100.00"
    },
    {
      "count": 7,
      "probability": 50,
      "revenue": 118500,
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "stage_name": "Proposal",
      "weighted_revenue": "59250.00"
    }
  ],
  "stalled": 6,
  "stalled_value": 58200,
  "total_opportunities": 3,
  "total_revenue": "1250.00",
  "total_weighted_revenue": "1250.00",
  "weighted_value": 188900,
  "won_this_month": 7,
  "won_value_this_month": 55200
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        open: Some(38),open_value: Some(412300),pipeline_id: Some("d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string()),stages: vec![serde_json::from_str::<serde_json::Value>(r#"{"count":9,"probability":10,"revenue":41000,"stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","weighted_revenue":"4100.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":8,"probability":25,"revenue":96400,"stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","stage_name":"Qualified","weighted_revenue":"24100.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":7,"probability":50,"revenue":118500,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","weighted_revenue":"59250.00"}"#).unwrap_or(serde_json::Value::Null)],stalled: Some(6),stalled_value: Some(58200),total_opportunities: 3,total_revenue: "1250.00".to_string(),total_weighted_revenue: "1250.00".to_string(),weighted_value: Some(188900),won_this_month: Some(7),won_value_this_month: Some(55200),
    })
}
