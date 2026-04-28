// User-owned controller for handler 'list_tax_rates'.

use crate::handlers::list_tax_rates::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaxRate;

#[handler(ListTaxRatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
