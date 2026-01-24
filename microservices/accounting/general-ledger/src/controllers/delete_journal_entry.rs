// User-owned controller for handler 'delete_journal_entry'.
use crate::handlers::delete_journal_entry::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
