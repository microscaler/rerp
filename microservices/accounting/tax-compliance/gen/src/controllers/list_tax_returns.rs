// User-owned controller for handler 'list_tax_returns'.

use crate::handlers::list_tax_returns::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaxReturn;

#[handler(ListTaxReturnsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
