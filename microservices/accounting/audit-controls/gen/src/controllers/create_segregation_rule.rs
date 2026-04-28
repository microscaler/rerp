// User-owned controller for handler 'create_segregation_rule'.

use crate::handlers::create_segregation_rule::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateSegregationRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        approver_role: "example".to_string(),
        id: "example".to_string(),
        initiator_role: "example".to_string(),
        service_name: "example".to_string(),
    }
}
