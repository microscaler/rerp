// Implementation stub for handler 'create_tax_payment'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::create_tax_payment::{Request, Response};

#[handler(CreateTaxPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        amount: 0.0,
        id: "".to_string(),
        paid_at: None,
        return_id: "".to_string(),
        status: serde_json::json!({}),
    }
}
