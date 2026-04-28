// Implementation stub for handler 'create_elimination_rule'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_consolidation_gen::handlers::create_elimination_rule::{Request, Response};

#[handler(CreateEliminationRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        active: false,
        group_id: "".to_string(),
        id: "".to_string(),
        name: "".to_string(),
        rule_type: None,
    }
}
