// User-owned controller for handler 'add_team_member'.

use crate::handlers::add_team_member::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddTeamMemberController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "assignment_domain": "Assignment domain",
    //   "assignment_enabled": false,
    //   "assignment_max": 3,
    //   "assignment_optout": false,
    //   "color": 3,
    //   "crm_team_id": "de6d6d22-a452-5aed-8a69-d5b24ab67a01",
    //   "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //   "lead_day_count": 3,
    //   "lead_month_count": 3,
    //   "quota_monthly": "1250.00",
    //   "quota_quarterly": "1250.00",
    //   "quota_yearly": "1250.00",
    //   "user_id": "alex",
    //   "user_name": "User name"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "assignment_domain": "Assignment domain",
  "assignment_enabled": false,
  "assignment_max": 3,
  "assignment_optout": false,
  "color": 3,
  "crm_team_id": "de6d6d22-a452-5aed-8a69-d5b24ab67a01",
  "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
  "lead_day_count": 3,
  "lead_month_count": 3,
  "quota_monthly": "1250.00",
  "quota_quarterly": "1250.00",
  "quota_yearly": "1250.00",
  "user_id": "alex",
  "user_name": "User name"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        assignment_domain: Some("Assignment domain".to_string()),
        assignment_enabled: false,
        assignment_max: 3,
        assignment_optout: Some(false),
        color: Some(3),
        crm_team_id: "de6d6d22-a452-5aed-8a69-d5b24ab67a01".to_string(),
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(),
        lead_day_count: Some(3),
        lead_month_count: Some(3),
        quota_monthly: Some("1250.00".to_string()),
        quota_quarterly: Some("1250.00".to_string()),
        quota_yearly: Some("1250.00".to_string()),
        user_id: "alex".to_string(),
        user_name: Some("User name".to_string()),
    })
}
