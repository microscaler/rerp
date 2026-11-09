// User-owned controller for handler 'get_team_performance'.

use crate::handlers::get_team_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTeamPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "member_count": 3,
    //   "team_id": "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31",
    //   "team_name": "Team name",
    //   "total_opportunities": 3,
    //   "total_revenue": "1250.00",
    //   "weighted_revenue": "1250.00",
    //   "win_rate": 12.5
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "member_count": 3,
  "team_id": "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31",
  "team_name": "Team name",
  "total_opportunities": 3,
  "total_revenue": "1250.00",
  "weighted_revenue": "1250.00",
  "win_rate": 12.5
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        member_count: 3,
        team_id: "6ae2cdbd-063f-5ea8-bd10-82ddcee5cb31".to_string(),
        team_name: "Team name".to_string(),
        total_opportunities: 3,
        total_revenue: "1250.00".to_string(),
        weighted_revenue: "1250.00".to_string(),
        win_rate: 12.5,
    })
}
