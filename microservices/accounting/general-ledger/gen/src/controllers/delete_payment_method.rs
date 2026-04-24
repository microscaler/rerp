// User-owned controller for handler 'delete_payment_method'.

use crate::handlers::delete_payment_method::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeletePaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    }
}
