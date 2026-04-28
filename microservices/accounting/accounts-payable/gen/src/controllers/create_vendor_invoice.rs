// User-owned controller for handler 'create_vendor_invoice'.

use crate::handlers::create_vendor_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: Some(3.14),
        approval_status: "example".to_string(),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        description: Some("example".to_string()),
        due_date: Some("example".to_string()),
        id: "example".to_string(),
        invoice_date: Some("example".to_string()),
        invoice_id: "example".to_string(),
        invoice_number: Some("example".to_string()),
        net_amount: Some(3.14),
        payment_status: Some("example".to_string()),
        status: "example".to_string(),
        tax_amount: Some(3.14),
        terms: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        vendor_id: "example".to_string(),
    }
}
