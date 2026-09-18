// User-owned controller for handler 'list_tax_periods'.

use crate::handlers::list_tax_periods::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListTaxPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
