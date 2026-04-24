// User-owned controller for handler 'list_journal_entries_by_journal'.

use crate::handlers::list_journal_entries_by_journal::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JournalEntry;

#[handler(ListJournalEntriesByJournalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
