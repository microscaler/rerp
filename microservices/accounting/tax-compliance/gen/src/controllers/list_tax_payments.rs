// User-owned controller for handler 'list_tax_payments'.

use crate::handlers::list_tax_payments::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListTaxPaymentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
