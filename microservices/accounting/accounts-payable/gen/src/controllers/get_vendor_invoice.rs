// User-owned controller for handler 'get_vendor_invoice'.

use crate::handlers::get_vendor_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
