// User-owned controller for handler 'list_leads'.

use crate::handlers::list_leads::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Lead;

#[handler(ListLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
