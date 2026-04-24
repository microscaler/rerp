// User-owned controller for handler 'delete_fiscal_year'.

use crate::handlers::delete_fiscal_year::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteFiscalYearController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    }
}
