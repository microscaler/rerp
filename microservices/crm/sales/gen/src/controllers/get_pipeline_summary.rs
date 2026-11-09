// User-owned controller for handler 'get_pipeline_summary'.

use crate::handlers::get_pipeline_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        open: Some(42),
        open_value: Some(1.5),
        pipeline_id: Some("example".to_string()),
        stages: Some(vec![]),
        stalled: Some(42),
        stalled_value: Some(1.5),
        weighted_value: Some(1.5),
        won_this_month: Some(42),
        won_value_this_month: Some(1.5),
    })
}
