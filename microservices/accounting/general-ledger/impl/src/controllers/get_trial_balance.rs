// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::get_trial_balance::Request;
use serde_json::Value;

#[handler(GetTrialBalanceController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::ledger::get_trial_balance(
            executor,
            &context,
            &request.data.as_of_date,
            &request.data.currency_code,
            request.data.include_zero_balance,
        )
    }) {
        Ok(trial_balance) => HttpJson::new(200, trial_balance),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
