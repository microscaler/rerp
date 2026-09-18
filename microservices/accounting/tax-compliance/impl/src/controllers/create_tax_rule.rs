// Implementation stub for handler 'create_tax_rule'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::create_tax_rule::{Request, Response};

#[handler(CreateTaxRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        active: false,
        effective_from: "".to_string(),
        effective_to: None,
        id: "".to_string(),
        jurisdiction_code: "".to_string(),
        name: "".to_string(),
        rate: Some(0.0),
        tax_type: serde_json::json!({}),
    }
}
