// User-owned controller for handler 'delete_reconciliation'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_reconciliation::{Request, Response};

#[handler(DeleteReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
