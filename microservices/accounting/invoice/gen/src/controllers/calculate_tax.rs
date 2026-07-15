// User-owned controller for handler 'calculate_tax'.

use crate::handlers::calculate_tax::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaxBreakdown;

#[handler(CalculateTaxController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        breakdown: Some(vec![]),
        subtotal: Some(3.14),
        total: Some(3.14),
        total_tax: Some(3.14),
    })
}
