// User-owned controller for handler 'create_deferred_revenue'.

use crate::handlers::create_deferred_revenue::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDeferredRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        balance: 1.5,
        id: "example".to_string(),
        item_type: "example".to_string(),
        schedule_id: "example".to_string(),
    })
}
