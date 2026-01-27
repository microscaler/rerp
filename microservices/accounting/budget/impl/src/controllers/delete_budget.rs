// User-owned controller for handler 'delete_budget'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_budget_gen::handlers::delete_budget::{Request, Response};

#[handler(DeleteBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
