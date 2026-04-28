// User-owned controller for handler 'list_vendor_invoices'.

use crate::handlers::list_vendor_invoices::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::VendorInvoice;

#[handler(ListVendorInvoicesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
