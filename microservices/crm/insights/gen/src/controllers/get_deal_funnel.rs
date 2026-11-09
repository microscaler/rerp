// User-owned controller for handler 'get_deal_funnel'.

use crate::handlers::get_deal_funnel::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FunnelStage;

#[handler(GetDealFunnelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        currency: Some("example".to_string()),
        lost_count: 42,
        overall_conversion_pct: 1.5,
        pipeline_id: "example".to_string(),
        stages: vec![],
        won_count: 42,
        won_value: Some(1.5),
    })
}
