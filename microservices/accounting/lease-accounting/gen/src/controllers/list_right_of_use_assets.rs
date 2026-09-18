// User-owned controller for handler 'list_right_of_use_assets'.

use crate::handlers::list_right_of_use_assets::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListRightOfUseAssetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
