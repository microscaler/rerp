// User-owned controller for handler 'bind_workflow_parameters'.

use crate::handlers::bind_workflow_parameters::{Request, Response};
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

#[handler(BindWorkflowParametersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "access_group_ids": [
    //     "dc6ee68c-2507-556c-b8b1-5a79f0575f8c"
    //   ],
    //   "action_config": {},
    //   "action_type": "SEND_EMAIL",
    //   "active_enrolments": 3,
    //   "bindings": {
    //     "key": "Value"
    //   },
    //   "completed_enrolments": 3,
    //   "conditions": {
    //     "items": [
    //       {
    //         "operator": "EQ",
    //         "subject": "October product update"
    //       }
    //     ],
    //     "op": "AND"
    //   },
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "description": "Example",
    //   "entry_step_id": "93404b0b-da88-5a3f-9bbc-46f4030b5dc2",
    //   "exit_rules": {
    //     "on_conversion": false,
    //     "on_goal_reached": false,
    //     "on_unsubscribe": false
    //   },
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "is_active": true,
    //   "last_run": "2026-11-05T16:00:00Z",
    //   "name": "Northwind Traders",
    //   "notes": [
    //     {
    //       "text": "Thanks for getting in touch."
    //     }
    //   ],
    //   "recipe_id": "ext_2aa648fff9",
    //   "reentry": "ONCE",
    //   "run_count": 3,
    //   "sequence": 1,
    //   "status": "DRAFT",
    //   "tracking": {
    //     "conversion_id": "30ae9dc3-8338-5da8-b1c5-632528d89c09",
    //     "goal_id": "6f60683f-3bf5-5a28-b467-77189733bf5b"
    //   },
    //   "trigger_config": {},
    //   "trigger_type": "STAGE_CHANGE",
    //   "triggers": [
    //     {
    //       "type": "STAGE_CHANGE"
    //     }
    //   ],
    //   "unbound_parameters": [
    //     {
    //       "key": "snippet.signature",
    //       "kind": "LIST",
    //       "label": "Primary"
    //     }
    //   ],
    //   "unsubscribe_scope": "TRIGGER_LISTS",
    //   "updated_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "access_group_ids": [
    "dc6ee68c-2507-556c-b8b1-5a79f0575f8c"
  ],
  "action_config": {},
  "action_type": "SEND_EMAIL",
  "active_enrolments": 3,
  "bindings": {
    "key": "Value"
  },
  "completed_enrolments": 3,
  "conditions": {
    "items": [
      {
        "operator": "EQ",
        "subject": "October product update"
      }
    ],
    "op": "AND"
  },
  "created_at": "2026-11-05T16:00:00Z",
  "description": "Example",
  "entry_step_id": "93404b0b-da88-5a3f-9bbc-46f4030b5dc2",
  "exit_rules": {
    "on_conversion": false,
    "on_goal_reached": false,
    "on_unsubscribe": false
  },
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "is_active": true,
  "last_run": "2026-11-05T16:00:00Z",
  "name": "Northwind Traders",
  "notes": [
    {
      "text": "Thanks for getting in touch."
    }
  ],
  "recipe_id": "ext_2aa648fff9",
  "reentry": "ONCE",
  "run_count": 3,
  "sequence": 1,
  "status": "DRAFT",
  "tracking": {
    "conversion_id": "30ae9dc3-8338-5da8-b1c5-632528d89c09",
    "goal_id": "6f60683f-3bf5-5a28-b467-77189733bf5b"
  },
  "trigger_config": {},
  "trigger_type": "STAGE_CHANGE",
  "triggers": [
    {
      "type": "STAGE_CHANGE"
    }
  ],
  "unbound_parameters": [
    {
      "key": "snippet.signature",
      "kind": "LIST",
      "label": "Primary"
    }
  ],
  "unsubscribe_scope": "TRIGGER_LISTS",
  "updated_at": "2026-11-05T16:00:00Z"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        access_group_ids: Some(vec!["dc6ee68c-2507-556c-b8b1-5a79f0575f8c".to_string()]),action_config: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),action_type: Some(serde_json::from_value::<ActionType>(serde_json::Value::String("SEND_EMAIL".to_string())).unwrap_or_default()),active_enrolments: Some(3),bindings: Some(serde_json::from_str::<serde_json::Value>(r#"{"key":"Value"}"#).unwrap_or(serde_json::Value::Null)),completed_enrolments: Some(3),conditions: Some(serde_json::from_value::<ConditionGroup>(serde_json::json!({"items":[{"operator":"EQ","subject":"October product update"}],"op":"AND"})).unwrap_or_default()),created_at: "2026-11-05T16:00:00Z".to_string(),description: Some("Example".to_string()),entry_step_id: Some("93404b0b-da88-5a3f-9bbc-46f4030b5dc2".to_string()),exit_rules: Some(serde_json::from_value::<ExitRules>(serde_json::json!({"on_conversion":false,"on_goal_reached":false,"on_unsubscribe":false})).unwrap_or_default()),id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),is_active: true,last_run: Some("2026-11-05T16:00:00Z".to_string()),name: "Northwind Traders".to_string(),notes: Some(vec![serde_json::from_value::<StepNote>(serde_json::json!({"text":"Thanks for getting in touch."})).unwrap_or_default()]),recipe_id: Some("ext_2aa648fff9".to_string()),reentry: Some(serde_json::from_value::<ReentryRule>(serde_json::Value::String("ONCE".to_string())).unwrap_or_default()),run_count: Some(3),sequence: 1,status: Some("DRAFT".to_string()),tracking: Some(serde_json::from_str::<serde_json::Value>(r#"{"conversion_id":"30ae9dc3-8338-5da8-b1c5-632528d89c09","goal_id":"6f60683f-3bf5-5a28-b467-77189733bf5b"}"#).unwrap_or(serde_json::Value::Null)),trigger_config: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),trigger_type: Some(serde_json::from_value::<TriggerType>(serde_json::Value::String("STAGE_CHANGE".to_string())).unwrap_or_default()),triggers: Some(vec![serde_json::from_value::<WorkflowTrigger>(serde_json::json!({"type":"STAGE_CHANGE"})).unwrap_or_default()]),unbound_parameters: Some(vec![serde_json::from_value::<RecipeParameter>(serde_json::json!({"key":"snippet.signature","kind":"LIST","label":"Primary"})).unwrap_or_default()]),unsubscribe_scope: Some("TRIGGER_LISTS".to_string()),updated_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
