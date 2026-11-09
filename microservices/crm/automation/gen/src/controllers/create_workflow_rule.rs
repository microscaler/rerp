// User-owned controller for handler 'create_workflow_rule'.

use crate::handlers::create_workflow_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActionType;
#[allow(unused_imports)]
use crate::handlers::types::ConditionOperator;
#[allow(unused_imports)]
use crate::handlers::types::ConditionValueType;

#[handler(CreateWorkflowRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "condition_field": "Condition field",
    //   "condition_operator": "EQUALS",
    //   "created_at": "2026-06-08T16:00:00Z",
    //   "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //   "is_active": true,
    //   "sequence": 1,
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "condition_field": "Condition field",
  "condition_operator": "EQUALS",
  "created_at": "2026-06-08T16:00:00Z",
  "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
  "is_active": true,
  "sequence": 1,
  "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        action_config: Some(Default::default()),
        action_type: Some(Default::default()),
        condition_field: "Condition field".to_string(),
        condition_operator: serde_json::from_value::<ConditionOperator>(serde_json::Value::String(
            "EQUALS".to_string(),
        ))
        .unwrap_or_default(),
        condition_value: Some("example".to_string()),
        condition_value_type: Some(Default::default()),
        created_at: Some("2026-06-08T16:00:00Z".to_string()),
        id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
        is_active: true,
        sequence: 1,
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),
    })
}
