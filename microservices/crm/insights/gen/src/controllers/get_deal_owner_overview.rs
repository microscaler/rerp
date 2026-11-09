// User-owned controller for handler 'get_deal_owner_overview'.

use crate::handlers::get_deal_owner_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealOwnerRow;

#[handler(GetDealOwnerOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "currency": "EUR",
    //   "owners": [
    //     {
    //       "avg_days_to_close": 29,
    //       "avg_deal_value": "8070.00",
    //       "deals_created": 31,
    //       "lost_count": 5,
    //       "lost_value": "1250.00",
    //       "name": "Alex Morgan",
    //       "open_count": 16,
    //       "open_value": "1250.00",
    //       "tasks_completed": 96,
    //       "user_id": "alex",
    //       "win_rate": 0.71,
    //       "won_count": 12,
    //       "won_value": "96800.00"
    //     },
    //     {
    //       "avg_days_to_close": 38,
    //       "avg_deal_value": "7910.00",
    //       "deals_created": 27,
    //       "lost_count": 8,
    //       "lost_value": "1250.00",
    //       "name": "Sam Lee",
    //       "open_count": 10,
    //       "open_value": "1250.00",
    //       "tasks_completed": 64,
    //       "user_id": "sam",
    //       "win_rate": 0.53,
    //       "won_count": 9,
    //       "won_value": "71200.00"
    //     },
    //     {
    //       "avg_days_to_close": 41,
    //       "avg_deal_value": "4600.00",
    //       "deals_created": 12,
    //       "lost_count": 3,
    //       "lost_value": "1250.00",
    //       "name": "Riya Kapoor",
    //       "open_count": 3,
    //       "open_value": "1250.00",
    //       "tasks_completed": 28,
    //       "user_id": "riya",
    //       "win_rate": 0.57,
    //       "won_count": 4,
    //       "won_value": "18400.00"
    //     }
    //   ],
    //   "period": "QUARTER"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "currency": "EUR",
  "owners": [
    {
      "avg_days_to_close": 29,
      "avg_deal_value": "8070.00",
      "deals_created": 31,
      "lost_count": 5,
      "lost_value": "1250.00",
      "name": "Alex Morgan",
      "open_count": 16,
      "open_value": "1250.00",
      "tasks_completed": 96,
      "user_id": "alex",
      "win_rate": 0.71,
      "won_count": 12,
      "won_value": "96800.00"
    },
    {
      "avg_days_to_close": 38,
      "avg_deal_value": "7910.00",
      "deals_created": 27,
      "lost_count": 8,
      "lost_value": "1250.00",
      "name": "Sam Lee",
      "open_count": 10,
      "open_value": "1250.00",
      "tasks_completed": 64,
      "user_id": "sam",
      "win_rate": 0.53,
      "won_count": 9,
      "won_value": "71200.00"
    },
    {
      "avg_days_to_close": 41,
      "avg_deal_value": "4600.00",
      "deals_created": 12,
      "lost_count": 3,
      "lost_value": "1250.00",
      "name": "Riya Kapoor",
      "open_count": 3,
      "open_value": "1250.00",
      "tasks_completed": 28,
      "user_id": "riya",
      "win_rate": 0.57,
      "won_count": 4,
      "won_value": "18400.00"
    }
  ],
  "period": "QUARTER"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        currency: "EUR".to_string(),owners: vec![serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":29,"avg_deal_value":"8070.00","deals_created":31,"lost_count":5,"lost_value":"1250.00","name":"Alex Morgan","open_count":16,"open_value":"1250.00","tasks_completed":96,"user_id":"alex","win_rate":0.71,"won_count":12,"won_value":"96800.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":38,"avg_deal_value":"7910.00","deals_created":27,"lost_count":8,"lost_value":"1250.00","name":"Sam Lee","open_count":10,"open_value":"1250.00","tasks_completed":64,"user_id":"sam","win_rate":0.53,"won_count":9,"won_value":"71200.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":41,"avg_deal_value":"4600.00","deals_created":12,"lost_count":3,"lost_value":"1250.00","name":"Riya Kapoor","open_count":3,"open_value":"1250.00","tasks_completed":28,"user_id":"riya","win_rate":0.57,"won_count":4,"won_value":"18400.00"})).unwrap_or_default()],period: Some("QUARTER".to_string()),
    })
}
