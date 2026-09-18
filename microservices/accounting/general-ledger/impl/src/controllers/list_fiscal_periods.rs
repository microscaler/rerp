// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::list_fiscal_periods::Request;
use serde_json::Value;

#[handler(ListFiscalPeriodsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::ledger::list_fiscal_periods(
            executor,
            &context,
            request.data.state.as_deref(),
            request.data.from_date.as_deref(),
            request.data.to_date.as_deref(),
            request.data.limit,
        )
    }) {
        Ok(periods) => HttpJson::new(200, periods),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
