// User-owned controller for handler 'get_journal_entry'.

use crate::handlers::get_journal_entry::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JournalLine;

#[handler(GetJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        currency_code: "example".to_string(),
        entry_date: "example".to_string(),
        entry_number: "example".to_string(),
        fiscal_period_id: "example".to_string(),
        id: "example".to_string(),
        lines: vec![],
        posted_at: "example".to_string(),
        source_document_id: "example".to_string(),
        total_credit: "example".to_string(),
        total_debit: "example".to_string(),
    }
}
