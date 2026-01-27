// User-owned controller for handler 'list_depreciations'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_asset_gen::handlers::list_depreciations::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_asset_gen::handlers::types::Depreciation;

#[handler(ListDepreciationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(Default::default()),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
