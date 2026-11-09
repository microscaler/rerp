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
        activity_count: 3,
        avg_deal_size: "1250.00".to_string(),
        avg_sales_cycle_days: 12.5,
        leads_created: 3,
        opportunities_active: 3,
        opportunities_lost: 3,
        opportunities_won: 3,
        period: "Period".to_string(),
        quota_attainment: 12.5,
        revenue_expected: "1250.00".to_string(),
        revenue_won: "1250.00".to_string(),
        team_id: "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31".to_string(),
        user_id: "alex".to_string(),
        user_name: "User name".to_string(),
        weighted_pipeline: "1250.00".to_string(),
    })
}
