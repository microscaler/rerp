// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::create_ledger_account::Request;
use serde_json::Value;

#[handler(CreateLedgerAccountController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_write_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::accounts::create_ledger_account(executor, &context, &request.data)
    }) {
        Ok(creation) => HttpJson::new(if creation.created { 201 } else { 200 }, creation.account),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
