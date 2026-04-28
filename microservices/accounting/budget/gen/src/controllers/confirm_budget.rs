// User-owned controller for handler 'confirm_budget'.

use crate::handlers::confirm_budget::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ConfirmBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
