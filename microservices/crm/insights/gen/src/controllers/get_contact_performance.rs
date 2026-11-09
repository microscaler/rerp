// User-owned controller for handler 'get_contact_performance'.

use crate::handlers::get_contact_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ContactCounts;
#[allow(unused_imports)]
use crate::handlers::types::ContactGroupRow;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetContactPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        current: Default::default(),
        granularity: Some("example".to_string()),
        previous: Default::default(),
        top_lists: vec![],
        top_tags: vec![],
        trend: vec![],
    })
}
