// User-owned controller for handler 'delete_invoice_line'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::delete_invoice_line::{Request, Response};

#[handler(DeleteInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
