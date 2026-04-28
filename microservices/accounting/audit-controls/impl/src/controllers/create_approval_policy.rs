// Implementation stub for handler 'create_approval_policy'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_audit_controls_gen::handlers::create_approval_policy::{Request, Response};

#[handler(CreateApprovalPolicyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        action_name: "".to_string(),
        active: false,
        id: "".to_string(),
        minimum_approvals: Some(0),
        service_name: "".to_string(),
    }
}
