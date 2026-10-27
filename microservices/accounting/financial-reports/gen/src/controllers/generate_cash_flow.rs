// User-owned controller for handler 'generate_cash_flow'.

use crate::handlers::generate_cash_flow::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateCashFlowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_details: Some(vec![]),
        beginning_cash: Some(1.5),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        ending_cash: Some(1.5),
        financing_activities: Some(1.5),
        id: Some("example".to_string()),
        investing_activities: Some(1.5),
        net_cash_flow: 1.5,
        operating_activities: Some(1.5),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
    })
}
