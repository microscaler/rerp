// User-owned controller for handler 'update_vendor_payment'.

use crate::handlers::update_vendor_payment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateVendorPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        actual_payment_date: "example".to_string(),
        amount: 3.14,
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        gl_entry_id: "example".to_string(),
        id: "example".to_string(),
        notes: "example".to_string(),
        payment_date: "example".to_string(),
        payment_method: "example".to_string(),
        posted_to_gl: true,
        reference_number: "example".to_string(),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
        vendor_id: "example".to_string(),
    })
}
