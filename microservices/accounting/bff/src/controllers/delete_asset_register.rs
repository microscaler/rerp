// User-owned controller for handler 'delete_asset_register'.
use crate::handlers::delete_asset_register::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteAssetRegisterController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
