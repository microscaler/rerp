// User-owned controller for handler 'generate_cash_flow'.

use crate::handlers::generate_cash_flow::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateCashFlowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_details: Some(vec![]),
        beginning_cash: Some(3.14),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        ending_cash: Some(3.14),
        financing_activities: Some(3.14),
        id: Some("example".to_string()),
        investing_activities: Some(3.14),
        net_cash_flow: 3.14,
        operating_activities: Some(3.14),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
    })
}
