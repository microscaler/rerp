// User-owned controller for handler 'aging_summary'.

use crate::handlers::aging_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AgingSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        as_of_date: "example".to_string(),
        bad_debt_reserves: Some(1.5),
        bucket_1_30: 1.5,
        bucket_31_60: 1.5,
        bucket_61_90: 1.5,
        bucket_90_plus: 1.5,
        collection_efficiency_index: Some(1.5),
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        current: 1.5,
        days_sales_outstanding: Some(1.5),
        total_outstanding: 1.5,
    })
}
