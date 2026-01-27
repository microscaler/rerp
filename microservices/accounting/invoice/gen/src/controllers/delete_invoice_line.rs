// User-owned controller for handler 'delete_invoice_line'.
use crate::handlers::delete_invoice_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
