// User-owned controller for handler 'create_customer_invoice'.

use crate::handlers::create_customer_invoice::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        aging_bucket: "example".to_string(),
        company_id: "example".to_string(),
        created_at: "example".to_string(),
        credit_limit_check: true,
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        due_date: "example".to_string(),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        original_amount: 3.14,
        outstanding_amount: 3.14,
        status: "example".to_string(),
        terms: "example".to_string(),
        updated_at: "example".to_string(),
    })
}
