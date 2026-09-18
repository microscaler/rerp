// User-owned controller for handler 'list_lease_liabilities'.

use crate::handlers::list_lease_liabilities::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListLeaseLiabilitiesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
