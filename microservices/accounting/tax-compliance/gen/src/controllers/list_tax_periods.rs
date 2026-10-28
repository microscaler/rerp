// User-owned controller for handler 'list_tax_periods'.

use crate::handlers::list_tax_periods::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaxPeriod;

#[handler(ListTaxPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    })
}
