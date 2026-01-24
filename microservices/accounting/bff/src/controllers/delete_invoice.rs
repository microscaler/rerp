// User-owned controller for handler 'delete_invoice'.
use crate::handlers::delete_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
