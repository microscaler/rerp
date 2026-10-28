// User-owned controller for handler 'get_vendor_invoice'.

use crate::handlers::get_vendor_invoice::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ApprovalRecord;
#[allow(unused_imports)]
use crate::handlers::types::VendorInvoiceLineItem;
#[allow(unused_imports)]
use crate::handlers::types::VendorPayment;

#[handler(GetVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: "example".to_string(),
        approval_history: Some(vec![]),
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
        line_items: Some(vec![]),
        net_amount: "example".to_string(),
        payment_history: Some(vec![]),
        payment_status: "example".to_string(),
        status: "example".to_string(),
        tax_amount: "example".to_string(),
        terms: "example".to_string(),
        updated_at: Some("example".to_string()),
        vendor_id: "example".to_string(),
    })
}
