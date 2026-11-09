// User-owned controller for handler 'get_workflow_setup'.

use crate::handlers::get_workflow_setup::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::RecipeParameter;

#[handler(GetWorkflowSetupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        parameters: vec![],
        total: Some(42),
    })
}
