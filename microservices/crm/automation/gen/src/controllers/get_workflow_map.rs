// User-owned controller for handler 'get_workflow_map'.

use crate::handlers::get_workflow_map::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWorkflowMapController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: 42,
        completed: 42,
        goal_reached: Some(42),
        steps: vec![],
        stopped: Some(42),
        workflow_id: "example".to_string(),
    })
}
