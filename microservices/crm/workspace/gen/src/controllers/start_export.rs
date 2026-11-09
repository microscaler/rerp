// User-owned controller for handler 'start_export'.

use crate::handlers::start_export::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JobEntity;
#[allow(unused_imports)]
use crate::handlers::types::JobKind;
#[allow(unused_imports)]
use crate::handlers::types::JobOptions;
#[allow(unused_imports)]
use crate::handlers::types::JobStatus;

#[handler(StartExportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entity": "CONTACTS",
    //   "file_name": "contacts.csv",
    //   "id": "9b732034-b874-5185-911e-fd4355d5e7f0",
    //   "kind": "EXPORT",
    //   "options": {
    //     "format": "CSV"
    //   },
    //   "progress_pct": 0,
    //   "queued_at": "2026-11-05T16:00:00Z",
    //   "requested_by": "alex",
    //   "requested_by_name": "Alex Morgan",
    //   "source": "UI",
    //   "status": "QUEUED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "entity": "CONTACTS",
  "file_name": "contacts.csv",
  "id": "9b732034-b874-5185-911e-fd4355d5e7f0",
  "kind": "EXPORT",
  "options": {
    "format": "CSV"
  },
  "progress_pct": 0,
  "queued_at": "2026-11-05T16:00:00Z",
  "requested_by": "alex",
  "requested_by_name": "Alex Morgan",
  "source": "UI",
  "status": "QUEUED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_rows: Some(42),
        entity: serde_json::from_value::<JobEntity>(serde_json::Value::String(
            "CONTACTS".to_string(),
        ))
        .unwrap_or_default(),
        error_rows: Some(42),
        file_name: Some("contacts.csv".to_string()),
        finished_at: Some("example".to_string()),
        id: "9b732034-b874-5185-911e-fd4355d5e7f0".to_string(),
        integration_id: Some("example".to_string()),
        kind: serde_json::from_value::<JobKind>(serde_json::Value::String("EXPORT".to_string()))
            .unwrap_or_default(),
        message: Some("example".to_string()),
        options: Some(
            serde_json::from_value::<JobOptions>(serde_json::json!({"format":"CSV"}))
                .unwrap_or_default(),
        ),
        processed_rows: Some(42),
        progress_pct: Some(0.0),
        queued_at: "2026-11-05T16:00:00Z".to_string(),
        requested_by: "alex".to_string(),
        requested_by_name: Some("Alex Morgan".to_string()),
        result_expires_at: Some("example".to_string()),
        result_url: Some("example".to_string()),
        skipped_rows: Some(42),
        source: Some("UI".to_string()),
        started_at: Some("example".to_string()),
        status: serde_json::from_value::<JobStatus>(serde_json::Value::String(
            "QUEUED".to_string(),
        ))
        .unwrap_or_default(),
        total_rows: Some(42),
        updated_rows: Some(42),
    })
}
