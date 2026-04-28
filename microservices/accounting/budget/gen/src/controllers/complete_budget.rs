// User-owned controller for handler 'complete_budget'.

use crate::handlers::complete_budget::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CompleteBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
