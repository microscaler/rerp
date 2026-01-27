// User-owned controller for handler 'delete_depreciation'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_asset_gen::handlers::delete_depreciation::{Request, Response};

#[handler(DeleteDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
