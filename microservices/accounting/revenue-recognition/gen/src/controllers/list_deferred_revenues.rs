// User-owned controller for handler 'list_deferred_revenues'.

use crate::handlers::list_deferred_revenues::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListDeferredRevenuesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
