// User-owned controller for handler 'delete_vendor_invoice'.
use crate::handlers::delete_vendor_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
