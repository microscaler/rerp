// User-owned controller for handler 'delete_tax_repartition_line'.

use crate::handlers::delete_tax_repartition_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteTaxRepartitionLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    }
}
