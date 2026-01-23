// User-owned controller for handler 'delete_reconciliation'.
use crate::handlers::delete_reconciliation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
