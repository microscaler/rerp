// User-owned controller for handler 'create_payment_batch'.

use crate::handlers::create_payment_batch::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePaymentBatchController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        currency_code: "example".to_string(),
        id: "example".to_string(),
        payment_count: Some(42),
        payment_method: "example".to_string(),
        scheduled_payment_date: Some("example".to_string()),
        status: "example".to_string(),
        total_amount: 3.14,
    }
}
