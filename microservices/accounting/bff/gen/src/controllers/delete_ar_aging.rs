// User-owned controller for handler 'delete_ar_aging'.
use crate::handlers::delete_ar_aging::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
