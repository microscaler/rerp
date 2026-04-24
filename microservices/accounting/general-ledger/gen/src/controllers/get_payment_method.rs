// User-owned controller for handler 'get_payment_method'.

use crate::handlers::get_payment_method::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
