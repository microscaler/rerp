// User-owned controller for handler 'resume_asset_depreciation'.

use crate::handlers::resume_asset_depreciation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ResumeAssetDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
