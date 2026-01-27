// User-owned controller for handler 'delete_ap_aging'.
use crate::handlers::delete_ap_aging::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteApAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
