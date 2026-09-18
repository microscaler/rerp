// User-owned controller for handler 'get_customer_payment'.

use crate::handlers::get_customer_payment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCustomerPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: 3.14,
        applied_at: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        gl_entry_id: "example".to_string(),
        id: "example".to_string(),
        notes: "example".to_string(),
        payment_date: "example".to_string(),
        payment_method: "example".to_string(),
        posted_to_gl: true,
        reference_number: "example".to_string(),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
