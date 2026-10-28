// User-owned controller for handler 'get_open_item'.

use crate::handlers::get_open_item::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PaymentApplication;

#[handler(GetOpenItemController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        aging_bucket: "example".to_string(),
        company_id: "example".to_string(),
        created_at: "example".to_string(),
        credit_limit_check: Some(true),
        currency_code: "example".to_string(),
        customer_id: "example".to_string(),
        days_overdue: Some(42),
        due_date: Some("example".to_string()),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        original_amount: "example".to_string(),
        outstanding_amount: "example".to_string(),
        payment_history: Some(vec![]),
        status: "example".to_string(),
        terms: "example".to_string(),
        updated_at: "example".to_string(),
    })
}
