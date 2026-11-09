// User-owned controller for handler 'get_team_performance'.

use crate::handlers::get_team_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTeamPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        member_count: 42,
        team_id: "example".to_string(),
        team_name: "example".to_string(),
        total_opportunities: 42,
        total_revenue: rust_decimal::Decimal::new(12345, 2),
        weighted_revenue: rust_decimal::Decimal::new(12345, 2),
        win_rate: 1.5,
    })
}
