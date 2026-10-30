// User-owned controller for handler 'post_journal_entry'.

use crate::handlers::post_journal_entry::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JournalEntryLine;

#[handler(PostJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        currency_code: "example".to_string(),
        document_number: Some("example".to_string()),
        entry_date: "example".to_string(),
        entry_number: "example".to_string(),
        id: "example".to_string(),
        lines: vec![],
        reverses_entry_id: Some("example".to_string()),
        total_credit: "example".to_string(),
        total_debit: "example".to_string(),
    })
}
