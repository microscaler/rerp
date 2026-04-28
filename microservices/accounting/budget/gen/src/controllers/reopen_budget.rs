// User-owned controller for handler 'reopen_budget'.

use crate::handlers::reopen_budget::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReopenBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
