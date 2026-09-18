// User-owned controller for handler 'validate_tax_return'.

use crate::handlers::validate_tax_return::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ValidateTaxReturnController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        issues: vec![],
        return_id: "example".to_string(),
        valid: true,
    }
}
