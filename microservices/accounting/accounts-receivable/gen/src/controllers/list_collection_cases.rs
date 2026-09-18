// User-owned controller for handler 'list_collection_cases'.

use crate::handlers::list_collection_cases::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CollectionCase;

#[handler(ListCollectionCasesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
