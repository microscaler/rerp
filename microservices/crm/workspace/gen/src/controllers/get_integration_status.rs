// User-owned controller for handler 'get_integration_status'.

use crate::handlers::get_integration_status::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::IntegrationType;
#[allow(unused_imports)]
use crate::handlers::types::SyncDirection;
#[allow(unused_imports)]
use crate::handlers::types::SyncStatus;

#[handler(GetIntegrationStatusController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "config": {},
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //   "is_active": true,
    //   "last_sync": "2026-11-05T16:00:00Z",
    //   "name": "Northwind Traders",
    //   "sync_direction": "CRM_TO_EXTERNAL",
    //   "sync_status": "IDLE",
    //   "type": "ERP"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "config": {},
  "created_at": "2026-11-05T16:00:00Z",
  "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
  "is_active": true,
  "last_sync": "2026-11-05T16:00:00Z",
  "name": "Northwind Traders",
  "sync_direction": "CRM_TO_EXTERNAL",
  "sync_status": "IDLE",
  "type": "ERP"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        config: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: "94b293ed-258b-5091-8e79-75d2ac471f25".to_string(),
        is_active: Some(true),
        last_sync: Some("2026-11-05T16:00:00Z".to_string()),
        name: "Northwind Traders".to_string(),
        sync_direction: Some(
            serde_json::from_value::<SyncDirection>(serde_json::Value::String(
                "CRM_TO_EXTERNAL".to_string(),
            ))
            .unwrap_or_default(),
        ),
        sync_status: Some(
            serde_json::from_value::<SyncStatus>(serde_json::Value::String("IDLE".to_string()))
                .unwrap_or_default(),
        ),
        r#type: serde_json::from_value::<IntegrationType>(serde_json::Value::String(
            "ERP".to_string(),
        ))
        .unwrap_or_default(),
    })
}
