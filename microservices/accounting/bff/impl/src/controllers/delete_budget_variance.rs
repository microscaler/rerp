// User-owned controller for handler 'delete_budget_variance'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_budget_variance::{Request, Response};

#[handler(DeleteBudgetVarianceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
