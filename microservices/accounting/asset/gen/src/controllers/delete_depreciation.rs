// User-owned controller for handler 'delete_depreciation'.
use crate::handlers::delete_depreciation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
