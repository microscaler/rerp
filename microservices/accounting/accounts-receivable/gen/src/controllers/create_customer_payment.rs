// User-owned controller for handler 'create_customer_payment'.

use crate::handlers::create_customer_payment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCustomerPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: "example".to_string(),
        applied_at: Some("example".to_string()),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        notes: Some("example".to_string()),
        payment_date: "example".to_string(),
        payment_method: "example".to_string(),
        posted_to_gl: true,
        reference_number: Some("example".to_string()),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
