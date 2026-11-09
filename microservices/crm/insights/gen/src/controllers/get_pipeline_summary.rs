// User-owned controller for handler 'get_pipeline_summary'.

use crate::handlers::get_pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        stages: vec![],
        total_opportunities: 42,
        total_revenue: rust_decimal::Decimal::new(12345, 2),
        total_weighted_revenue: rust_decimal::Decimal::new(12345, 2),
    })
}
