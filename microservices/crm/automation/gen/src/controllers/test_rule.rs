// User-owned controller for handler 'test_rule'.

use crate::handlers::test_rule::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "actions_to_fire": [
    //     "Actions to fire"
    //   ],
    //   "condition_met": false,
    //   "matching_conditions": [
    //     "Matching conditions"
    //   ],
    //   "rule_id": "9fd16524-6914-54dc-a830-024bec1c8df0"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "actions_to_fire": [
    "Actions to fire"
  ],
  "condition_met": false,
  "matching_conditions": [
    "Matching conditions"
  ],
  "rule_id": "9fd16524-6914-54dc-a830-024bec1c8df0"
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        actions_to_fire: vec!["Actions to fire".to_string()],
        condition_met: false,
        matching_conditions: vec!["Matching conditions".to_string()],
        rule_id: "9fd16524-6914-54dc-a830-024bec1c8df0".to_string(),
    })
}
