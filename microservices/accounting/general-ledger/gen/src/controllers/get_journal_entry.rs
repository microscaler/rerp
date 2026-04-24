// User-owned controller for handler 'get_journal_entry'.

use crate::handlers::get_journal_entry::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
