// User-owned controller for handler 'list_consolidation_groups'.

use crate::handlers::list_consolidation_groups::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListConsolidationGroupsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
