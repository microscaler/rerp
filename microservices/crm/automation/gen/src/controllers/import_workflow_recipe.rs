// User-owned controller for handler 'import_workflow_recipe'.

use crate::handlers::import_workflow_recipe::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::RecipeParameter;
#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(ImportWorkflowRecipeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        category: "example".to_string(),
        definition: Some(Default::default()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        industry: Some("example".to_string()),
        name: "example".to_string(),
        needs_templates: Some(42),
        parameters: Some(vec![]),
        source_url: Some("example".to_string()),
        step_count: Some(42),
        trigger_type: Some(Default::default()),
        trigger_types: Some(vec![]),
    })
}
