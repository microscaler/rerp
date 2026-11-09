// User-owned controller for handler 'update_workflow'.

use crate::handlers::update_workflow::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActionType;
#[allow(unused_imports)]
use crate::handlers::types::ExitRules;
#[allow(unused_imports)]
use crate::handlers::types::RecipeParameter;
#[allow(unused_imports)]
use crate::handlers::types::ReentryRule;
#[allow(unused_imports)]
use crate::handlers::types::StepNote;
#[allow(unused_imports)]
use crate::handlers::types::TriggerType;
#[allow(unused_imports)]
use crate::handlers::types::WorkflowTrigger;

#[handler(UpdateWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        access_group_ids: Some(vec![]),
        action_config: Some(Default::default()),
        action_type: Some(Default::default()),
        active_enrolments: Some(42),
        bindings: Some(Default::default()),
        completed_enrolments: Some(42),
        conditions: Some(Default::default()),
        created_at: "example".to_string(),
        description: Some("example".to_string()),
        entry_step_id: Some("example".to_string()),
        exit_rules: Some(Default::default()),
        id: "example".to_string(),
        is_active: true,
        last_run: Some("example".to_string()),
        name: "example".to_string(),
        notes: Some(vec![]),
        recipe_id: Some("example".to_string()),
        reentry: Some(Default::default()),
        run_count: Some(42),
        sequence: 42,
        status: Some("example".to_string()),
        tracking: Some(Default::default()),
        trigger_config: Some(Default::default()),
        trigger_type: Some(Default::default()),
        triggers: Some(vec![]),
        unbound_parameters: Some(vec![]),
        unsubscribe_scope: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    })
}
