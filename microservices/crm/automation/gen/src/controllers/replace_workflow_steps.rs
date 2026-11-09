// User-owned controller for handler 'replace_workflow_steps'.

use crate::handlers::replace_workflow_steps::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkflowStep;

#[handler(ReplaceWorkflowStepsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        entry_step_id: "example".to_string(),
        steps: vec![],
        version: Some(42),
        workflow_id: "example".to_string(),
    })
}
