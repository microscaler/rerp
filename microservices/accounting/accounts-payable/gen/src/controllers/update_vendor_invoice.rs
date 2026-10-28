// User-owned controller for handler 'update_vendor_invoice'.

use crate::handlers::update_vendor_invoice::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: 1.5,
        approval_status: "example".to_string(),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        company_id: "example".to_string(),
        created_at: "example".to_string(),
        currency_code: "example".to_string(),
        description: Some("example".to_string()),
        due_date: "example".to_string(),
        id: "example".to_string(),
        invoice_date: "example".to_string(),
        invoice_id: "example".to_string(),
        invoice_number: "example".to_string(),
        net_amount: 1.5,
        payment_status: "example".to_string(),
        status: "example".to_string(),
        tax_amount: 1.5,
        terms: "example".to_string(),
        updated_at: Some("example".to_string()),
        vendor_id: "example".to_string(),
    })
}
