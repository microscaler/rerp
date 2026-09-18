// Implementation stub for handler 'create_tax_return'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::create_tax_return::{Request, Response};

#[handler(CreateTaxReturnController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        company_id: "".to_string(),
        currency_code: None,
        id: "".to_string(),
        period_id: "".to_string(),
        return_type: "".to_string(),
        status: serde_json::json!({}),
        submitted_at: None,
        total_due: 0.0,
    }
}
