// User-owned controller for handler 'list_tax_returns'.

use crate::handlers::list_tax_returns::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListTaxReturnsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
