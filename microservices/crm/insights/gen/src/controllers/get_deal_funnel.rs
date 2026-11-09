// User-owned controller for handler 'get_deal_funnel'.

use crate::handlers::get_deal_funnel::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FunnelStage;

#[handler(GetDealFunnelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "currency": "EUR",
    //   "lost_count": 28,
    //   "overall_conversion_pct": 0.3,
    //   "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //   "stages": [
    //     {
    //       "avg_days_in_stage": 3.1,
    //       "current": 9,
    //       "entered": 122,
    //       "lost_from_stage": 9,
    //       "name": "New",
    //       "progressed": 84,
    //       "progression_pct": 0.69,
    //       "sequence": 1,
    //       "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "value": "41000.00"
    //     },
    //     {
    //       "avg_days_in_stage": 6.4,
    //       "current": 8,
    //       "entered": 84,
    //       "lost_from_stage": 12,
    //       "name": "Qualified",
    //       "progressed": 51,
    //       "progression_pct": 0.61,
    //       "sequence": 2,
    //       "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
    //       "value": "96400.00"
    //     },
    //     {
    //       "avg_days_in_stage": 9.8,
    //       "current": 7,
    //       "entered": 51,
    //       "lost_from_stage": 4,
    //       "name": "Proposal",
    //       "progressed": 39,
    //       "progression_pct": 0.76,
    //       "sequence": 3,
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "value": "118500.00"
    //     }
    //   ],
    //   "won_count": 36,
    //   "won_value": "186400.00"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "currency": "EUR",
  "lost_count": 28,
  "overall_conversion_pct": 0.3,
  "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
  "stages": [
    {
      "avg_days_in_stage": 3.1,
      "current": 9,
      "entered": 122,
      "lost_from_stage": 9,
      "name": "New",
      "progressed": 84,
      "progression_pct": 0.69,
      "sequence": 1,
      "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "value": "41000.00"
    },
    {
      "avg_days_in_stage": 6.4,
      "current": 8,
      "entered": 84,
      "lost_from_stage": 12,
      "name": "Qualified",
      "progressed": 51,
      "progression_pct": 0.61,
      "sequence": 2,
      "stage_id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
      "value": "96400.00"
    },
    {
      "avg_days_in_stage": 9.8,
      "current": 7,
      "entered": 51,
      "lost_from_stage": 4,
      "name": "Proposal",
      "progressed": 39,
      "progression_pct": 0.76,
      "sequence": 3,
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "value": "118500.00"
    }
  ],
  "won_count": 36,
  "won_value": "186400.00"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        currency: Some("EUR".to_string()),lost_count: 28,overall_conversion_pct: 0.3,pipeline_id: "d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string(),stages: vec![serde_json::from_value::<FunnelStage>(serde_json::json!({"avg_days_in_stage":3.1,"current":9,"entered":122,"lost_from_stage":9,"name":"New","progressed":84,"progression_pct":0.69,"sequence":1,"stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"41000.00"})).unwrap_or_default(), serde_json::from_value::<FunnelStage>(serde_json::json!({"avg_days_in_stage":6.4,"current":8,"entered":84,"lost_from_stage":12,"name":"Qualified","progressed":51,"progression_pct":0.61,"sequence":2,"stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"96400.00"})).unwrap_or_default(), serde_json::from_value::<FunnelStage>(serde_json::json!({"avg_days_in_stage":9.8,"current":7,"entered":51,"lost_from_stage":4,"name":"Proposal","progressed":39,"progression_pct":0.76,"sequence":3,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"118500.00"})).unwrap_or_default()],won_count: 36,won_value: Some("186400.00".to_string()),
    })
}
