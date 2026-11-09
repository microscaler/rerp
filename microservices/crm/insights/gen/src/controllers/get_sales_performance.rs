// User-owned controller for handler 'get_sales_performance'.

use crate::handlers::get_sales_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealStageRow;
#[allow(unused_imports)]
use crate::handlers::types::SalesPerformanceRow;

#[handler(GetSalesPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        currency: Some("example".to_string()),
        owners: vec![],
        period: Some("example".to_string()),
        pipeline_id: "example".to_string(),
        stages: vec![],
    })
}
