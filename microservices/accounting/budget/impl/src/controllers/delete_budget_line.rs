// User-owned controller for handler 'delete_budget_line'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_budget_gen::handlers::delete_budget_line::{Request, Response};

#[handler(DeleteBudgetLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
