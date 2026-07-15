// User-owned controller for handler 'register_invoice_payment'.

use crate::handlers::register_invoice_payment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RegisterInvoicePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: 3.14,
        currency_code: "example".to_string(),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        status: "example".to_string(),
    })
}
