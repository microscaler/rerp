// User-owned controller for handler 'list_stages'.

use crate::handlers::list_stages::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Stage;

#[handler(ListStagesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
