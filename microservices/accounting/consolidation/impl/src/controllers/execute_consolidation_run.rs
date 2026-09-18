// Implementation stub for handler 'execute_consolidation_run'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_consolidation_gen::handlers::execute_consolidation_run::{Request, Response};

#[handler(ExecuteConsolidationRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        executed_at: None,
        fiscal_period_id: "".to_string(),
        group_id: "".to_string(),
        id: "".to_string(),
        status: serde_json::json!({}),
    }
}
