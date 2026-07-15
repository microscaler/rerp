// User-owned controller for handler 'collections_summary'.

use crate::handlers::collections_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CollectionsSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_type: Some(Default::default()),
        response_rate: Some(3.14),
        total_activities: Some(42),
        upcoming_follow_ups: Some(42),
    })
}
