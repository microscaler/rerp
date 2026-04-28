// User-owned controller for handler 'create_budget'.

use crate::handlers::create_budget::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
