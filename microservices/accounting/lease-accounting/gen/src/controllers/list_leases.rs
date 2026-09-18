// User-owned controller for handler 'list_leases'.

use crate::handlers::list_leases::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListLeasesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
