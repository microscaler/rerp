// User-owned controller for handler 'create_tax_rule'.

use crate::handlers::create_tax_rule::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        effective_from: "example".to_string(),
        effective_to: Some("example".to_string()),
        id: "example".to_string(),
        jurisdiction_code: "example".to_string(),
        name: "example".to_string(),
        rate: Some(3.14),
        tax_type: Default::default(),
    }
}
