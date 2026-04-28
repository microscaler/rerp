// User-owned controller for handler 'register_vendor_invoice_payment'.

use crate::handlers::register_vendor_invoice_payment::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RegisterVendorInvoicePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        actual_payment_date: Some("example".to_string()),
        amount: 3.14,
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        notes: Some("example".to_string()),
        payment_date: "example".to_string(),
        payment_method: "example".to_string(),
        posted_to_gl: Some(true),
        reference_number: Some("example".to_string()),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
        vendor_id: Some("example".to_string()),
    }
}
