// User-owned controller for handler 'get_payment'.

use crate::handlers::get_payment::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        applied_at: Some("example".to_string()),
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        customer_id: Some("example".to_string()),
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        notes: Some("example".to_string()),
        payment_date: "example".to_string(),
        payment_method: "example".to_string(),
        posted_to_gl: Some(true),
        reference_number: Some("example".to_string()),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
    }
}
