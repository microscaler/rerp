// User-owned controller for handler 'get_weighted_pipeline'.

use crate::handlers::get_weighted_pipeline::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWeightedPipelineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_team: vec![],
        by_user: vec![],
    })
}
