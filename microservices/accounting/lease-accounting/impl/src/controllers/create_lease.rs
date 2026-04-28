// Implementation stub for handler 'create_lease'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_lease_accounting_gen::handlers::create_lease::{Request, Response};

#[handler(CreateLeaseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        classification: serde_json::json!({}),
        commencement_date: "".to_string(),
        discount_rate: Some(0.0),
        id: "".to_string(),
        lease_number: "".to_string(),
        status: serde_json::json!({}),
        termination_date: None,
    }
}
