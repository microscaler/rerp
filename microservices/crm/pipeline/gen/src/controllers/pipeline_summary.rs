// User-owned controller for handler 'pipeline_summary'.

use crate::handlers::pipeline_summary::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        stages: vec![],
        total_opportunities: 42,
        total_revenue: 1.5,
        total_weighted_revenue: 1.5,
    }
}
