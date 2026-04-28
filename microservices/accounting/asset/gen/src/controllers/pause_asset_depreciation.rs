// User-owned controller for handler 'pause_asset_depreciation'.

use crate::handlers::pause_asset_depreciation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PauseAssetDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
