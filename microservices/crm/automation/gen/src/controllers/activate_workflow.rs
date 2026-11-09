// User-owned controller for handler 'activate_workflow'.

use crate::handlers::activate_workflow::{Request, Response};
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

#[handler(ActivateWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active_enrolments": 1204,
    //   "completed_enrolments": 8311,
    //   "created_at": "2026-06-08T16:00:00Z",
    //   "description": "",
    //   "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //   "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //   "is_active": true,
    //   "last_run": "2026-11-05T15:58:00Z",
    //   "name": "Welcome series (7 days)",
    //   "reentry": "ONCE",
    //   "sequence": 1,
    //   "status": "ACTIVE",
    //   "trigger_type": "SUBSCRIBED_TO_LIST",
    //   "triggers": [
    //     {
    //       "config": {
    //         "label": "Newsletter"
    //       },
    //       "type": "SUBSCRIBED_TO_LIST"
    //     },
    //     {
    //       "config": {
    //         "label": "Website footer"
    //       },
    //       "type": "FORM_SUBMITTED"
    //     }
    //   ],
    //   "updated_at": "2026-11-02T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "active_enrolments": 1204,
  "completed_enrolments": 8311,
  "created_at": "2026-06-08T16:00:00Z",
  "description": "",
  "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
  "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
  "is_active": true,
  "last_run": "2026-11-05T15:58:00Z",
  "name": "Welcome series (7 days)",
  "reentry": "ONCE",
  "sequence": 1,
  "status": "ACTIVE",
  "trigger_type": "SUBSCRIBED_TO_LIST",
  "triggers": [
    {
      "config": {
        "label": "Newsletter"
      },
      "type": "SUBSCRIBED_TO_LIST"
    },
    {
      "config": {
        "label": "Website footer"
      },
      "type": "FORM_SUBMITTED"
    }
  ],
  "updated_at": "2026-11-02T16:00:00Z"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        access_group_ids: Some(vec![]),
        action_config: Some(Default::default()),
        action_type: Some(Default::default()),
        active_enrolments: Some(1204),
        bindings: Some(Default::default()),
        completed_enrolments: Some(8311),
        conditions: Some(Default::default()),
        created_at: "2026-06-08T16:00:00Z".to_string(),
        description: Some("".to_string()),
        entry_step_id: Some("f1189b89-3e27-5826-bc93-e33f7450d394".to_string()),
        exit_rules: Some(Default::default()),
        id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
        is_active: true,
        last_run: Some("2026-11-05T15:58:00Z".to_string()),
        name: "Welcome series (7 days)".to_string(),
        notes: Some(vec![]),
        recipe_id: Some("example".to_string()),
        reentry: Some(
            serde_json::from_value::<ReentryRule>(serde_json::Value::String("ONCE".to_string()))
                .unwrap_or_default(),
        ),
        run_count: Some(42),
        sequence: 1,
        status: Some("ACTIVE".to_string()),
        tracking: Some(Default::default()),
        trigger_config: Some(Default::default()),
        trigger_type: Some(
            serde_json::from_value::<TriggerType>(serde_json::Value::String(
                "SUBSCRIBED_TO_LIST".to_string(),
            ))
            .unwrap_or_default(),
        ),
        triggers: Some(vec![
            serde_json::from_value::<WorkflowTrigger>(
                serde_json::json!({"config":{"label":"Newsletter"},"type":"SUBSCRIBED_TO_LIST"}),
            )
            .unwrap_or_default(),
            serde_json::from_value::<WorkflowTrigger>(
                serde_json::json!({"config":{"label":"Website footer"},"type":"FORM_SUBMITTED"}),
            )
            .unwrap_or_default(),
        ]),
        unbound_parameters: Some(vec![]),
        unsubscribe_scope: Some("example".to_string()),
        updated_at: Some("2026-11-02T16:00:00Z".to_string()),
    })
}
