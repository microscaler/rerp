// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::reverse_journal_entry::Request;
use serde_json::Value;

#[handler(ReverseJournalEntryController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_write_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::journals::reverse_journal_entry(executor, &context, &request.data)
    }) {
        Ok(change) => HttpJson::new(if change.created { 201 } else { 200 }, change.entry),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
