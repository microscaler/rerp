// User-owned controller for handler 'delete_payment_term'.

use crate::handlers::delete_payment_term::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeletePaymentTermController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    })
}
