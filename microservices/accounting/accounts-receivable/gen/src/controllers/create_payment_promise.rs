// User-owned controller for handler 'create_payment_promise'.

use crate::handlers::create_payment_promise::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePaymentPromiseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        customer_id: "example".to_string(),
        id: "example".to_string(),
        promised_amount: 3.14,
        promised_date: "example".to_string(),
        status: "example".to_string(),
    })
}
