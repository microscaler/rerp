// User-owned controller for handler 'get_fiscal_year'.

use crate::handlers::get_fiscal_year::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetFiscalYearController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
