// User-owned controller for handler 'list_lease_modifications'.

use crate::handlers::list_lease_modifications::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListLeaseModificationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
