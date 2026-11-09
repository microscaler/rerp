// User-owned controller for handler 'list_goals'.

use crate::handlers::list_goals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Goal;

#[handler(ListGoalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
