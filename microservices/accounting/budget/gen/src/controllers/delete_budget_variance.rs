// User-owned controller for handler 'delete_budget_variance'.
use crate::handlers::delete_budget_variance::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteBudgetVarianceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
