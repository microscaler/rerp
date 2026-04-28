// User-owned controller for handler 'create_asset'.

use crate::handlers::create_asset::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
