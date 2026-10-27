// User-owned controller for handler 'list_lease_liabilities'.

use crate::handlers::list_lease_liabilities::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LeaseLiability;

#[handler(ListLeaseLiabilitiesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
