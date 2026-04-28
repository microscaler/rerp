// User-owned controller for handler 'get_asset'.

use crate::handlers::get_asset::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
