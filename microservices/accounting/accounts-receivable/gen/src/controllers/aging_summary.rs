// User-owned controller for handler 'aging_summary'.

use crate::handlers::aging_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AgingSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        as_of_date: "example".to_string(),
        bucket_1_30: "example".to_string(),
        bucket_31_60: "example".to_string(),
        bucket_61_90: "example".to_string(),
        bucket_90_plus: "example".to_string(),
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        current: "example".to_string(),
        total_outstanding: "example".to_string(),
    })
}
