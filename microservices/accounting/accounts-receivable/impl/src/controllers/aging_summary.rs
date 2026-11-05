// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::aging_summary::Request;
use serde_json::Value;

#[handler(AgingSummaryController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let (company_id, customer_id, as_of) = match crate::ledger::parse_aging_query(
        query.company_id.as_deref(),
        query.customer_id.as_deref(),
        query.as_of_date.as_deref(),
    ) {
        Ok(scope) => scope,
        Err(error) => return crate::http_support::ar_error(error),
    };
    match crate::http_support::with_ar_transaction(&context, |executor| {
        crate::ledger::aging_summary(executor, &context, company_id, customer_id, as_of)
    }) {
        Ok(summary) => HttpJson::new(200, summary),
        Err(error) => crate::http_support::ar_error(error),
    }
}
