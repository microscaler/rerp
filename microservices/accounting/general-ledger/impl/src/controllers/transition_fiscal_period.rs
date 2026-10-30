// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::transition_fiscal_period::Request;
use serde_json::Value;

#[handler(TransitionFiscalPeriodController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_write_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::periods::transition_fiscal_period(executor, &context, &request.data)
    }) {
        Ok(change) => HttpJson::new(if change.created { 201 } else { 200 }, change.period),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
