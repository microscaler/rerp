// User-owned controller for handler 'instantiate_workflow_recipe'.

use crate::handlers::instantiate_workflow_recipe::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActionType;
#[allow(unused_imports)]
use crate::handlers::types::ConditionGroup;
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

#[handler(InstantiateWorkflowRecipeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "description": "Five emails over seven days after a list subscription, ending early when the contact activates.",
    //   "id": "6b579986-98a1-52bc-b486-8c48e0820541",
    //   "is_active": true,
    //   "name": "Welcome series",
    //   "sequence": 1,
    //   "trigger_type": "SUBSCRIBED_TO_LIST"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "created_at": "2026-11-05T16:00:00Z",
  "description": "Five emails over seven days after a list subscription, ending early when the contact activates.",
  "id": "6b579986-98a1-52bc-b486-8c48e0820541",
  "is_active": true,
  "name": "Welcome series",
  "sequence": 1,
  "trigger_type": "SUBSCRIBED_TO_LIST"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        access_group_ids: Some(vec![]),action_config: Some(Default::default()),action_type: Some(Default::default()),active_enrolments: Some(42),bindings: Some(Default::default()),completed_enrolments: Some(42),conditions: Some(Default::default()),created_at: "2026-11-05T16:00:00Z".to_string(),description: Some("Five emails over seven days after a list subscription, ending early when the contact activates.".to_string()),entry_step_id: Some("example".to_string()),exit_rules: Some(Default::default()),id: "6b579986-98a1-52bc-b486-8c48e0820541".to_string(),is_active: true,last_run: Some("example".to_string()),name: "Welcome series".to_string(),notes: Some(vec![]),recipe_id: Some("example".to_string()),reentry: Some(Default::default()),run_count: Some(42),sequence: 1,status: Some("example".to_string()),tracking: Some(Default::default()),trigger_config: Some(Default::default()),trigger_type: Some(serde_json::from_value::<TriggerType>(serde_json::Value::String("SUBSCRIBED_TO_LIST".to_string())).unwrap_or_default()),triggers: Some(vec![]),unbound_parameters: Some(vec![]),unsubscribe_scope: Some("example".to_string()),updated_at: Some("example".to_string()),
    })
}
