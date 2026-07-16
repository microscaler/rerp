// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::list_ledger_accounts::Request;
use serde_json::Value;

#[handler(ListLedgerAccountsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::ledger::list_accounts(
            executor,
            &context,
            request.data.account_type.as_deref(),
            request.data.currency_code.as_deref(),
            request.data.active,
            request.data.limit,
        )
    }) {
        Ok(accounts) => HttpJson::new(200, accounts),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
