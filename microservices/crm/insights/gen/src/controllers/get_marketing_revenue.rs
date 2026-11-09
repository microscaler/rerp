// User-owned controller for handler 'get_marketing_revenue'.

use crate::handlers::get_marketing_revenue::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;
#[allow(unused_imports)]
use crate::handlers::types::RevenueSourceRow;

#[handler(GetMarketingRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        attribution_window_days: 42,
        currency: "example".to_string(),
        from_automation_messages: 1.5,
        from_campaigns: 1.5,
        series: Some(vec![]),
        top_automation_messages: Some(vec![]),
        top_campaigns: Some(vec![]),
        total_revenue: rust_decimal::Decimal::new(12345, 2),
    })
}
