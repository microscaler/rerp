// User-owned controller for handler 'create_approval_policy'.

use crate::handlers::create_approval_policy::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateApprovalPolicyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        action_name: "example".to_string(),
        active: true,
        id: "example".to_string(),
        minimum_approvals: Some(42),
        service_name: "example".to_string(),
    }
}
