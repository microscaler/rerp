// Implementation stub for handler 'create_tax_period'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::create_tax_period::{Request, Response};

#[handler(CreateTaxPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        company_id: "".to_string(),
        due_date: None,
        end_date: "".to_string(),
        id: "".to_string(),
        jurisdiction_code: "".to_string(),
        start_date: "".to_string(),
        status: serde_json::json!({}),
        tax_type: serde_json::json!({}),
    }
}
