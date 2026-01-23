// User-owned controller for handler 'deleteJournalEntry'.
use crate::handlers::deleteJournalEntry::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
