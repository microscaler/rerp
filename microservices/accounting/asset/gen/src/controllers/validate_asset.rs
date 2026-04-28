// User-owned controller for handler 'validate_asset'.

use crate::handlers::validate_asset::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ValidateAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
