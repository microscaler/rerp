// User-owned controller for handler 'delete_payment'.
use crate::handlers::delete_payment::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeletePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
