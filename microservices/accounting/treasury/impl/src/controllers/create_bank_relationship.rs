// Implementation stub for handler 'create_bank_relationship'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_treasury_gen::handlers::create_bank_relationship::{Request, Response};

#[handler(CreateBankRelationshipController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        active: false,
        bank_name: "".to_string(),
        id: "".to_string(),
        relationship_type: "".to_string(),
    }
}
