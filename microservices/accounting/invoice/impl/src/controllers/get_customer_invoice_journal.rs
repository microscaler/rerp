// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::get_customer_invoice_journal::Request;
use serde_json::Value;

#[handler(GetCustomerInvoiceJournalController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let document_id = match crate::posting::parse_uuid(&request.data.id, "id") {
        Ok(id) => id,
        Err(error) => return crate::http_support::posting_error(error),
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::posting::get_journal(executor, document_id)
    }) {
        Ok(journal) => HttpJson::new(200, crate::posting::journal_json(&journal)),
        Err(error) => crate::http_support::posting_error(error),
    }
}
