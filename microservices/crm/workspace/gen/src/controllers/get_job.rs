// User-owned controller for handler 'get_job'.

use crate::handlers::get_job::{Request, Response};
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

#[handler(GetJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entity": "DEALS",
    //   "file_name": "deals-q4.xlsx",
    //   "finished_at": "2026-11-05T16:00:00Z",
    //   "id": "628f717a-6de9-522e-9f1d-93e1c15c19f7",
    //   "kind": "EXPORT",
    //   "options": {
    //     "filter": {
    //       "items": [
    //         {
    //           "operator": "EQ",
    //           "subject": "October product update"
    //         }
    //       ],
    //       "op": "AND"
    //     },
    //     "format": "XLSX"
    //   },
    //   "processed_rows": 24,
    //   "progress_pct": 62,
    //   "queued_at": "2026-11-05T15:57:00Z",
    //   "requested_by": "alex",
    //   "requested_by_name": "Alex Morgan",
    //   "source": "UI",
    //   "started_at": "2026-11-05T15:58:00Z",
    //   "status": "RUNNING",
    //   "total_rows": 38
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "entity": "DEALS",
  "file_name": "deals-q4.xlsx",
  "finished_at": "2026-11-05T16:00:00Z",
  "id": "628f717a-6de9-522e-9f1d-93e1c15c19f7",
  "kind": "EXPORT",
  "options": {
    "filter": {
      "items": [
        {
          "operator": "EQ",
          "subject": "October product update"
        }
      ],
      "op": "AND"
    },
    "format": "XLSX"
  },
  "processed_rows": 24,
  "progress_pct": 62,
  "queued_at": "2026-11-05T15:57:00Z",
  "requested_by": "alex",
  "requested_by_name": "Alex Morgan",
  "source": "UI",
  "started_at": "2026-11-05T15:58:00Z",
  "status": "RUNNING",
  "total_rows": 38
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_rows: Some(42),entity: serde_json::from_value::<JobEntity>(serde_json::Value::String("DEALS".to_string())).unwrap_or_default(),error_rows: Some(42),file_name: Some("deals-q4.xlsx".to_string()),finished_at: Some("2026-11-05T16:00:00Z".to_string()),id: "628f717a-6de9-522e-9f1d-93e1c15c19f7".to_string(),integration_id: Some("example".to_string()),kind: serde_json::from_value::<JobKind>(serde_json::Value::String("EXPORT".to_string())).unwrap_or_default(),message: Some("example".to_string()),options: Some(serde_json::from_value::<JobOptions>(serde_json::json!({"filter":{"items":[{"operator":"EQ","subject":"October product update"}],"op":"AND"},"format":"XLSX"})).unwrap_or_default()),processed_rows: Some(24),progress_pct: Some(62.0),queued_at: "2026-11-05T15:57:00Z".to_string(),requested_by: "alex".to_string(),requested_by_name: Some("Alex Morgan".to_string()),result_expires_at: Some("example".to_string()),result_url: Some("example".to_string()),skipped_rows: Some(42),source: Some("UI".to_string()),started_at: Some("2026-11-05T15:58:00Z".to_string()),status: serde_json::from_value::<JobStatus>(serde_json::Value::String("RUNNING".to_string())).unwrap_or_default(),total_rows: Some(38),updated_rows: Some(42),
    })
}
