// User-owned controller for handler 'get_time_to_close'.

use crate::handlers::get_time_to_close::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTimeToCloseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_source: vec![],
        by_stage: vec![],
        overall_avg_days: 1.5,
    })
}
