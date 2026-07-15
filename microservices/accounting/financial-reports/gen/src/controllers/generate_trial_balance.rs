// User-owned controller for handler 'generate_trial_balance'.

use crate::handlers::generate_trial_balance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateTrialBalanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accounts: Some(vec![]),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        id: Some("example".to_string()),
        is_balanced: Some(true),
        period_end: "example".to_string(),
        total_credits: 3.14,
        total_debits: 3.14,
    })
}
