// Implementation stub for handler 'create_recognition_rule'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_revenue_recognition_gen::handlers::create_recognition_rule::{Request, Response};

#[handler(CreateRecognitionRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        active: false,
        id: "".to_string(),
        method: serde_json::json!({}),
        name: "".to_string(),
    }
}
