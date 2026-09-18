// User-owned controller for handler 'list_control_exceptions'.

use crate::handlers::list_control_exceptions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListControlExceptionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
