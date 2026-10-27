// User-owned controller for handler 'cash_flow_forecast'.

use crate::handlers::cash_flow_forecast::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CashFlowForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_day: Some(vec![]),
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        net_cash_flow: Some(1.5),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        total_commitments: 1.5,
        total_payments: 1.5,
    })
}
