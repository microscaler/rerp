// User-owned controller for handler 'list_consolidation_runs'.

use crate::handlers::list_consolidation_runs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConsolidationRun;

#[handler(ListConsolidationRunsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
