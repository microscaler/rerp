// User-owned controller for handler 'delete_asset'.

use crate::handlers::delete_asset::Request;
use brrtrouter::typed::HttpNoContent;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpNoContent {
    HttpNoContent
}
