// User-owned controller for handler 'get_campaign_performance'.

use crate::handlers::get_campaign_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::OutreachOverview;
#[allow(unused_imports)]
use crate::handlers::types::PaginatedCampaignPerformanceRow;

#[handler(GetCampaignPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        campaigns: Default::default(),
        comparison: Some(Default::default()),
        comparison_basis: Some("example".to_string()),
        totals: Default::default(),
    })
}
