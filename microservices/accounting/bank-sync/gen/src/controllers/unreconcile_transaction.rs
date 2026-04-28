// User-owned controller for handler 'unreconcile_transaction'.

use crate::handlers::unreconcile_transaction::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UnreconcileTransactionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
