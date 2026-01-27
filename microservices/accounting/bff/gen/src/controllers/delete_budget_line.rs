// User-owned controller for handler 'delete_budget_line'.
use crate::handlers::delete_budget_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteBudgetLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
