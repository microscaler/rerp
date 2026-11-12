// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::send_customer_statements::Request;
use serde_json::Value;

#[handler(SendCustomerStatementsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    // One REPEATABLE READ transaction: the frozen render model is replayed
    // from the journal and written next to the job row, so the accepted
    // statement can never be a torn view of concurrent postings.
    match crate::http_support::with_ar_snapshot_transaction(&context, |executor| {
        crate::statements::send(
            executor,
            &context,
            &query.customer_ids,
            &query.statement_date,
            query.delivery_method.as_deref(),
        )
    }) {
        Ok(acceptance) => HttpJson::new(202, acceptance),
        Err(error) => crate::http_support::ar_error(error),
    }
}
