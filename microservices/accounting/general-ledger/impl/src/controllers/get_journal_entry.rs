// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::get_journal_entry::Request;
use serde_json::Value;

#[handler(GetJournalEntryController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let journal_id = match crate::ledger::parse_uuid(&request.data.id, "id") {
        Ok(id) => id,
        Err(error) => return crate::http_support::ledger_error(error),
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::ledger::get_journal_entry(executor, &context, journal_id)
    }) {
        Ok(journal) => HttpJson::new(200, journal),
        Err(error) => crate::http_support::ledger_error(error),
    }
}
