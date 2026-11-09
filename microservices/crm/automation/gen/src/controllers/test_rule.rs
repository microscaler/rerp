// User-owned controller for handler 'test_rule'.

use crate::handlers::test_rule::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        actions_to_fire: vec![],
        condition_met: true,
        matching_conditions: vec![],
        rule_id: "example".to_string(),
    })
}
