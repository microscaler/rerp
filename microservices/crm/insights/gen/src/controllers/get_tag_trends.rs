// User-owned controller for handler 'get_tag_trends'.

use crate::handlers::get_tag_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;
#[allow(unused_imports)]
use crate::handlers::types::TagTrendRow;

#[handler(GetTagTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        avg_per_day: 1.5,
        most_used_in_range: Some(vec![]),
        most_used_overall: Some(vec![]),
        series: vec![],
        tags: vec![],
        tags_applied: 42,
    })
}
