// User-owned controller for handler 'get_invoice'.

use crate::handlers::get_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
