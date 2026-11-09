// User-owned controller for handler 'update_team'.

use crate::handlers::update_team::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateTeamController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active_lead_count": 3,
    //   "alias_name": "Alias name",
    //   "assignment_auto_enabled": false,
    //   "assignment_domain": "Assignment domain",
    //   "assignment_enabled": false,
    //   "assignment_max": 3,
    //   "assignment_optout": false,
    //   "create_date": "2026-11-05T16:00:00Z",
    //   "description": "Example",
    //   "emoji": "Emoji",
    //   "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //   "member_count": 3,
    //   "name": "Northwind Traders",
    //   "privacy_security": "MY_TEAM",
    //   "total_revenue": "1250.00",
    //   "use_leads": false,
    //   "use_opportunities": false
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "active_lead_count": 3,
  "alias_name": "Alias name",
  "assignment_auto_enabled": false,
  "assignment_domain": "Assignment domain",
  "assignment_enabled": false,
  "assignment_max": 3,
  "assignment_optout": false,
  "create_date": "2026-11-05T16:00:00Z",
  "description": "Example",
  "emoji": "Emoji",
  "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
  "member_count": 3,
  "name": "Northwind Traders",
  "privacy_security": "MY_TEAM",
  "total_revenue": "1250.00",
  "use_leads": false,
  "use_opportunities": false
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active_lead_count: Some(3),
        alias_name: Some("Alias name".to_string()),
        assignment_auto_enabled: Some(false),
        assignment_domain: Some("Assignment domain".to_string()),
        assignment_enabled: false,
        assignment_max: Some(3),
        assignment_optout: Some(false),
        create_date: Some("2026-11-05T16:00:00Z".to_string()),
        description: Some("Example".to_string()),
        emoji: Some("Emoji".to_string()),
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(),
        member_count: Some(3),
        name: "Northwind Traders".to_string(),
        privacy_security: Some("MY_TEAM".to_string()),
        total_revenue: Some("1250.00".to_string()),
        use_leads: Some(false),
        use_opportunities: Some(false),
    })
}
