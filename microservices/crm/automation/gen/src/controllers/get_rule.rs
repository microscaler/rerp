// User-owned controller for handler 'get_rule'.

use crate::handlers::get_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActionType;
#[allow(unused_imports)]
use crate::handlers::types::ConditionOperator;
#[allow(unused_imports)]
use crate::handlers::types::ConditionValueType;

#[handler(GetRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "action_config": {},
    //   "action_type": "SEND_EMAIL",
    //   "condition_field": "Condition field",
    //   "condition_operator": "EQUALS",
    //   "condition_value": "Condition value",
    //   "condition_value_type": "TEXT",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "is_active": true,
    //   "sequence": 1,
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "action_config": {},
  "action_type": "SEND_EMAIL",
  "condition_field": "Condition field",
  "condition_operator": "EQUALS",
  "condition_value": "Condition value",
  "condition_value_type": "TEXT",
  "created_at": "2026-11-05T16:00:00Z",
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "is_active": true,
  "sequence": 1,
  "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        action_config: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        action_type: Some(
            serde_json::from_value::<ActionType>(serde_json::Value::String(
                "SEND_EMAIL".to_string(),
            ))
            .unwrap_or_default(),
        ),
        condition_field: "Condition field".to_string(),
        condition_operator: serde_json::from_value::<ConditionOperator>(serde_json::Value::String(
            "EQUALS".to_string(),
        ))
        .unwrap_or_default(),
        condition_value: Some("Condition value".to_string()),
        condition_value_type: Some(
            serde_json::from_value::<ConditionValueType>(serde_json::Value::String(
                "TEXT".to_string(),
            ))
            .unwrap_or_default(),
        ),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),
        is_active: true,
        sequence: 1,
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),
    })
}
