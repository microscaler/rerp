// User-owned controller for handler 'delete_asset'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_asset_gen::handlers::delete_asset::{Request, Response};

#[handler(DeleteAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
