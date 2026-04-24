// User-owned controller for handler 'get_payment_term'.

use crate::handlers::get_payment_term::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPaymentTermController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
