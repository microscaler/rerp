// User-owned controller for handler 'get_customer_invoice'.

use crate::handlers::get_customer_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PostedInvoiceLine;
#[allow(unused_imports)]
use crate::handlers::types::SourceReference;

#[handler(GetCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        discount_amount: "example".to_string(),
        document_number: "example".to_string(),
        document_type: "example".to_string(),
        due_date: "example".to_string(),
        id: "example".to_string(),
        invoice_date: "example".to_string(),
        lines: vec![],
        original_document_id: Some("example".to_string()),
        posted_at: "example".to_string(),
        rounding_minor_units: 42,
        source: Default::default(),
        status: "example".to_string(),
        subtotal: "example".to_string(),
        tax_amount: "example".to_string(),
        total_amount: "example".to_string(),
    }
}
