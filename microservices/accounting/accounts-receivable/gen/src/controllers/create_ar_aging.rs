// User-owned controller for handler 'create_ar_aging'.

use crate::handlers::create_ar_aging::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        as_of_date: "example".to_string(),
        bucket_1_30: 3.14,
        bucket_31_60: 3.14,
        bucket_61_90: 3.14,
        bucket_90_plus: 3.14,
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        current: 3.14,
        customer_id: "example".to_string(),
        id: "example".to_string(),
        invoice_count: Some(42),
        total_outstanding: Some(3.14),
    }
}
