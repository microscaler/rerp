// User-owned controller for handler 'deleteAccount'.
use crate::handlers::deleteAccount::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
