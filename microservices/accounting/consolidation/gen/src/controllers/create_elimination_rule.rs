// User-owned controller for handler 'create_elimination_rule'.

use crate::handlers::create_elimination_rule::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateEliminationRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        group_id: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        rule_type: Some("example".to_string()),
    }
}
