// User-owned controller for handler 'submit_tax_return'.

use crate::handlers::submit_tax_return::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SubmitTaxReturnController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        external_reference: Some("example".to_string()),
        id: "example".to_string(),
        return_id: "example".to_string(),
        status: "example".to_string(),
    }
}
