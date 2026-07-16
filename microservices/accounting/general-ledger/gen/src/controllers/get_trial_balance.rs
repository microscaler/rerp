// User-owned controller for handler 'get_trial_balance'.

use crate::handlers::get_trial_balance::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TrialBalanceLine;

#[handler(GetTrialBalanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        as_of_date: "example".to_string(),
        balanced: true,
        currency_code: "example".to_string(),
        difference: "example".to_string(),
        lines: vec![],
        total_credit: "example".to_string(),
        total_debit: "example".to_string(),
    }
}
