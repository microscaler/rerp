// User-owned controller for handler 'list_consolidation_runs'.

use crate::handlers::list_consolidation_runs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListConsolidationRunsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
