// User-owned controller for handler 'reconcile_journal_items'.

use crate::handlers::reconcile_journal_items::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReconcileJournalItemsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        id: "example".to_string(),
        journal_item_ids: vec![],
        status: "example".to_string(),
    }
}
