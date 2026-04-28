// User-owned controller for handler 'list_signature_requests'.

use crate::handlers::list_signature_requests::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListSignatureRequestsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
