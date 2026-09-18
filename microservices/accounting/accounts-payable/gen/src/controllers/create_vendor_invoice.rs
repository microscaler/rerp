// User-owned controller for handler 'create_vendor_invoice'.

use crate::handlers::create_vendor_invoice::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: 3.14,
        approval_status: "example".to_string(),
        approved_at: "example".to_string(),
        approved_by: "example".to_string(),
        company_id: "example".to_string(),
        created_at: "example".to_string(),
        currency_code: "example".to_string(),
        description: "example".to_string(),
        due_date: "example".to_string(),
        id: "example".to_string(),
        invoice_date: "example".to_string(),
        invoice_id: "example".to_string(),
        invoice_number: "example".to_string(),
        net_amount: 3.14,
        payment_status: "example".to_string(),
        status: "example".to_string(),
        tax_amount: 3.14,
        terms: "example".to_string(),
        updated_at: Some("example".to_string()),
        vendor_id: "example".to_string(),
    })
}
