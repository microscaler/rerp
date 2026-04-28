// User-owned controller for handler 'create_payment_application'.

use crate::handlers::create_payment_application::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePaymentApplicationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        applied_amount: 3.14,
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        customer_invoice_id: "example".to_string(),
        id: "example".to_string(),
        payment_id: "example".to_string(),
    }
}
