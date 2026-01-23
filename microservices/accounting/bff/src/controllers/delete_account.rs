// User-owned controller for handler 'delete_account'.
use crate::handlers::delete_account::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
