// User-owned controller for handler 'get_win_loss_analysis'.

use crate::handlers::get_win_loss_analysis::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWinLossAnalysisController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        lost_count: 42,
        lost_reasons: vec![],
        period: "example".to_string(),
        revenue_won: rust_decimal::Decimal::new(12345, 2),
        total_closed: 42,
        win_rate: 1.5,
        won_count: 42,
    })
}
