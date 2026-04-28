// User-owned controller for handler 'list_elimination_entries'.

use crate::handlers::list_elimination_entries::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEliminationEntriesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
