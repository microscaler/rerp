// User-owned controller for handler 'list_team_members'.

use crate::handlers::list_team_members::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TeamMember;

#[handler(ListTeamMembersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "assignment_enabled": false,
    //       "assignment_max": 3,
    //       "crm_team_id": "de6d6d22-a452-5aed-8a69-d5b24ab67a01",
    //       "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //       "user_id": "alex"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "assignment_enabled": false,
      "assignment_max": 3,
      "crm_team_id": "de6d6d22-a452-5aed-8a69-d5b24ab67a01",
      "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
      "user_id": "alex"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<TeamMember>(serde_json::json!({"assignment_enabled":false,"assignment_max":3,"crm_team_id":"de6d6d22-a452-5aed-8a69-d5b24ab67a01","id":"1b23e5c3-0ff4-5d6d-a54b-e72da751ce26","user_id":"alex"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
