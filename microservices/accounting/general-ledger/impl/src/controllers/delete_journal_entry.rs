// User-owned controller for handler 'delete_journal_entry'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::delete_journal_entry::{Request, Response};

#[handler(DeleteJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
