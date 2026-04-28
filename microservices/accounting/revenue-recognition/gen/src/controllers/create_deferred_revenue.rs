// User-owned controller for handler 'create_deferred_revenue'.

use crate::handlers::create_deferred_revenue::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDeferredRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        balance: 3.14,
        id: "example".to_string(),
        item_type: "example".to_string(),
        schedule_id: "example".to_string(),
    }
}
