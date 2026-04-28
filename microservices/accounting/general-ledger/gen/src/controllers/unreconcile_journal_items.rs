// User-owned controller for handler 'unreconcile_journal_items'.

use crate::handlers::unreconcile_journal_items::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UnreconcileJournalItemsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
