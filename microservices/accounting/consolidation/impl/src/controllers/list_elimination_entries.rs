// Implementation stub for handler 'list_elimination_entries'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_consolidation_gen::handlers::list_elimination_entries::{Request, Response};

#[handler(ListEliminationEntriesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
    }
}
