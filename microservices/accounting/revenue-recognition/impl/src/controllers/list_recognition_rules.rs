// Implementation stub for handler 'list_recognition_rules'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_revenue_recognition_gen::handlers::list_recognition_rules::{Request, Response};

#[handler(ListRecognitionRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
    }
}
