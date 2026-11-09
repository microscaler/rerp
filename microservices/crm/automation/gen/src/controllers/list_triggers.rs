// User-owned controller for handler 'list_triggers'.

use crate::handlers::list_triggers::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Trigger;

#[handler(ListTriggersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
