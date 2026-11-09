// User-owned controller for handler 'get_deliverability_report'.

use crate::handlers::get_deliverability_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetDeliverabilityReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_domain: Some(vec![]),
        by_provider: Some(vec![]),
        complaint_rate: Some(1.5),
        delivered_rate: Some(1.5),
        domain_reputation: Some("example".to_string()),
        from: Some("example".to_string()),
        hard_bounce_rate: Some(1.5),
        soft_bounce_rate: Some(1.5),
        suppressed_30d: Some(42),
        to: Some("example".to_string()),
    })
}
