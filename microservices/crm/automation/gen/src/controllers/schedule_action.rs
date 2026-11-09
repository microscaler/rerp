// User-owned controller for handler 'schedule_action'.

use crate::handlers::schedule_action::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ExecutionStatus;

#[handler(ScheduleActionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "action_config": {},
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "executed_at": "2026-11-05T16:00:00Z",
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "scheduled_at": "2026-11-05T16:00:00Z",
    //   "status": "PENDING",
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "action_config": {},
  "created_at": "2026-11-05T16:00:00Z",
  "executed_at": "2026-11-05T16:00:00Z",
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "scheduled_at": "2026-11-05T16:00:00Z",
  "status": "PENDING",
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
        action_config: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        executed_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),
        scheduled_at: "2026-11-05T16:00:00Z".to_string(),
        status: serde_json::from_value::<ExecutionStatus>(serde_json::Value::String(
            "PENDING".to_string(),
        ))
        .unwrap_or_default(),
        workflow_id: Some("7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string()),
    })
}
