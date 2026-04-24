// User-owned controller for handler 'list_journal_entry_lines'.

use crate::handlers::list_journal_entry_lines::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JournalEntryLine;

#[handler(ListJournalEntryLinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response(vec![])
}
