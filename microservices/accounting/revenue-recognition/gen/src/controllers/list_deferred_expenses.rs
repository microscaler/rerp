// User-owned controller for handler 'list_deferred_expenses'.

use crate::handlers::list_deferred_expenses::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListDeferredExpensesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
