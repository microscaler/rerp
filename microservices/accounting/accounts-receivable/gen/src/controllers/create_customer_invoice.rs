// User-owned controller for handler 'create_customer_invoice'.

use crate::handlers::create_customer_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        aging_bucket: Some("example".to_string()),
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        credit_limit_check: Some(true),
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        due_date: Some("example".to_string()),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        original_amount: Some(3.14),
        outstanding_amount: Some(3.14),
        status: "example".to_string(),
        terms: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    }
}
