// User-owned controller for handler 'get_account_hierarchy'.

use crate::handlers::get_account_hierarchy::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountHierarchyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        limit: 42,
        page: 42,
        total: 42,
    }
}
