// User-owned controller for handler 'list_depreciations'.
use crate::handlers::list_depreciations::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Depreciation;

#[handler(ListDepreciationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
