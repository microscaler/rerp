// User-owned controller for handler 'get_rep_performance'.

use crate::handlers::get_rep_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetRepPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "activity_count": 3,
    //   "avg_deal_size": "1250.00",
    //   "avg_sales_cycle_days": 12.5,
    //   "items": [
    //     {
    //       "avg_deal": 8070,
    //       "lost_90d": 5,
    //       "open": 16,
    //       "owner_id": "alex",
    //       "owner_name": "Alex Morgan",
    //       "pipeline_value": 201400,
    //       "tasks_overdue": 2,
    //       "weighted": 98200,
    //       "win_rate": 0.71,
    //       "won_90d": 12,
    //       "won_value_90d": 96800
    //     },
    //     {
    //       "avg_deal": 7910,
    //       "lost_90d": 8,
    //       "open": 14,
    //       "owner_id": "sam",
    //       "owner_name": "Sam Lee",
    //       "pipeline_value": 156900,
    //       "tasks_overdue": 0,
    //       "weighted": 64100,
    //       "win_rate": 0.53,
    //       "won_90d": 9,
    //       "won_value_90d": 71200
    //     },
    //     {
    //       "avg_deal": 4600,
    //       "lost_90d": 3,
    //       "open": 8,
    //       "owner_id": "riya",
    //       "owner_name": "Riya Kapoor",
    //       "pipeline_value": 54000,
    //       "tasks_overdue": 1,
    //       "weighted": 26600,
    //       "win_rate": 0.57,
    //       "won_90d": 4,
    //       "won_value_90d": 18400
    //     }
    //   ],
    //   "leads_created": 3,
    //   "opportunities_active": 3,
    //   "opportunities_lost": 3,
    //   "opportunities_won": 3,
    //   "period": "Period",
    //   "quota_attainment": 12.5,
    //   "revenue_expected": "1250.00",
    //   "revenue_won": "1250.00",
    //   "team_id": "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31",
    //   "user_id": "alex",
    //   "user_name": "User name",
    //   "weighted_pipeline": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "activity_count": 3,
  "avg_deal_size": "1250.00",
  "avg_sales_cycle_days": 12.5,
  "items": [
    {
      "avg_deal": 8070,
      "lost_90d": 5,
      "open": 16,
      "owner_id": "alex",
      "owner_name": "Alex Morgan",
      "pipeline_value": 201400,
      "tasks_overdue": 2,
      "weighted": 98200,
      "win_rate": 0.71,
      "won_90d": 12,
      "won_value_90d": 96800
    },
    {
      "avg_deal": 7910,
      "lost_90d": 8,
      "open": 14,
      "owner_id": "sam",
      "owner_name": "Sam Lee",
      "pipeline_value": 156900,
      "tasks_overdue": 0,
      "weighted": 64100,
      "win_rate": 0.53,
      "won_90d": 9,
      "won_value_90d": 71200
    },
    {
      "avg_deal": 4600,
      "lost_90d": 3,
      "open": 8,
      "owner_id": "riya",
      "owner_name": "Riya Kapoor",
      "pipeline_value": 54000,
      "tasks_overdue": 1,
      "weighted": 26600,
      "win_rate": 0.57,
      "won_90d": 4,
      "won_value_90d": 18400
    }
  ],
  "leads_created": 3,
  "opportunities_active": 3,
  "opportunities_lost": 3,
  "opportunities_won": 3,
  "period": "Period",
  "quota_attainment": 12.5,
  "revenue_expected": "1250.00",
  "revenue_won": "1250.00",
  "team_id": "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31",
  "user_id": "alex",
  "user_name": "User name",
  "weighted_pipeline": "1250.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        activity_count: 3,avg_deal_size: "1250.00".to_string(),avg_sales_cycle_days: 12.5,items: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"avg_deal":8070,"lost_90d":5,"open":16,"owner_id":"alex","owner_name":"Alex Morgan","pipeline_value":201400,"tasks_overdue":2,"weighted":98200,"win_rate":0.71,"won_90d":12,"won_value_90d":96800}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"avg_deal":7910,"lost_90d":8,"open":14,"owner_id":"sam","owner_name":"Sam Lee","pipeline_value":156900,"tasks_overdue":0,"weighted":64100,"win_rate":0.53,"won_90d":9,"won_value_90d":71200}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"avg_deal":4600,"lost_90d":3,"open":8,"owner_id":"riya","owner_name":"Riya Kapoor","pipeline_value":54000,"tasks_overdue":1,"weighted":26600,"win_rate":0.57,"won_90d":4,"won_value_90d":18400}"#).unwrap_or(serde_json::Value::Null)]),leads_created: 3,opportunities_active: 3,opportunities_lost: 3,opportunities_won: 3,period: "Period".to_string(),quota_attainment: 12.5,revenue_expected: "1250.00".to_string(),revenue_won: "1250.00".to_string(),team_id: "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31".to_string(),user_id: "alex".to_string(),user_name: "User name".to_string(),weighted_pipeline: "1250.00".to_string(),
    })
}
