// User-owned controller for handler 'bulk_approve_journal_entries'.

use crate::handlers::bulk_approve_journal_entries::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BulkApproveJournalEntriesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        failed: 42,
        results: vec![],
        succeeded: 42,
        total: 42,
    }
}
