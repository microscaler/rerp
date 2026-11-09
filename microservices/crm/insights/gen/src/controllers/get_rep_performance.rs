// User-owned controller for handler 'get_rep_performance'.

use crate::handlers::get_rep_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetRepPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        activity_count: 42,
        avg_deal_size: 1.5,
        avg_sales_cycle_days: 1.5,
        leads_created: 42,
        opportunities_active: 42,
        opportunities_lost: 42,
        opportunities_won: 42,
        period: "example".to_string(),
        quota_attainment: 1.5,
        revenue_expected: rust_decimal::Decimal::new(12345, 2),
        revenue_won: rust_decimal::Decimal::new(12345, 2),
        team_id: "example".to_string(),
        user_id: "example".to_string(),
        user_name: "example".to_string(),
        weighted_pipeline: 1.5,
    })
}
