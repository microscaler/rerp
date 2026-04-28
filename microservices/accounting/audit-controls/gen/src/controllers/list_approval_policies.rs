// User-owned controller for handler 'list_approval_policies'.

use crate::handlers::list_approval_policies::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListApprovalPoliciesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
