// User-owned controller for handler 'perform_vendor_invoice_three_way_match'.

use crate::handlers::perform_vendor_invoice_three_way_match::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PerformVendorInvoiceThreeWayMatchController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        matched: true,
        variance_count: 42,
        variances: Some(vec![]),
        vendor_invoice_id: "example".to_string(),
    })
}
