// User-owned controller for handler 'update_trigger'.

use crate::handlers::update_trigger::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(UpdateTriggerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "enabled": true,
    //   "entity": "Entity",
    //   "field": "Field",
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "last_fired": "2026-11-05T16:00:00Z",
    //   "schedule": "Schedule",
    //   "type": "STAGE_CHANGE",
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "created_at": "2026-11-05T16:00:00Z",
  "enabled": true,
  "entity": "Entity",
  "field": "Field",
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "last_fired": "2026-11-05T16:00:00Z",
  "schedule": "Schedule",
  "type": "STAGE_CHANGE",
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
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        enabled: true,
        entity: "Entity".to_string(),
        field: Some("Field".to_string()),
        id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),
        last_fired: Some("2026-11-05T16:00:00Z".to_string()),
        schedule: Some("Schedule".to_string()),
        r#type: serde_json::from_value::<TriggerType>(serde_json::Value::String(
            "STAGE_CHANGE".to_string(),
        ))
        .unwrap_or_default(),
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),
    })
}
