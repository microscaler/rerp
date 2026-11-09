// User-owned controller for handler 'get_live_visitors'.

use crate::handlers::get_live_visitors::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetLiveVisitorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        anonymous_count: Some(42),
        items: vec![],
    })
}
