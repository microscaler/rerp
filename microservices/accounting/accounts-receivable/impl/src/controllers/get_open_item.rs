// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::get_open_item::Request;
use serde_json::Value;

#[handler(GetOpenItemController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_ar_transaction(&context, |executor| {
        crate::ledger::get_open_item(executor, &context, &request.data.id)
    }) {
        Ok(item) => HttpJson::new(200, item),
        Err(error) => crate::http_support::ar_error(error),
    }
}
