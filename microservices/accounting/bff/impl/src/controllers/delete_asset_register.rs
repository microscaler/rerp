// User-owned controller for handler 'delete_asset_register'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_asset_register::{Request, Response};

#[handler(DeleteAssetRegisterController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
