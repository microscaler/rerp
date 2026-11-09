// User-owned controller for handler 'get_import_job'.

use crate::handlers::get_import_job::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetImportJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created": 3,
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "download_url": "https://northwindtraders.example/pricing",
    //   "errors": [
    //     {}
    //   ],
    //   "finished_at": "2026-11-05T16:00:00Z",
    //   "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //   "kind": "IMPORT",
    //   "processed_rows": 3,
    //   "skipped": 3,
    //   "status": "QUEUED",
    //   "total_rows": 3,
    //   "updated": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created": 3,
  "created_at": "2026-11-05T16:00:00Z",
  "download_url": "https://northwindtraders.example/pricing",
  "errors": [
    {}
  ],
  "finished_at": "2026-11-05T16:00:00Z",
  "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
  "kind": "IMPORT",
  "processed_rows": 3,
  "skipped": 3,
  "status": "QUEUED",
  "total_rows": 3,
  "updated": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created: Some(3),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        download_url: Some("https://northwindtraders.example/pricing".to_string()),
        errors: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        finished_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: "44f5daeb-faac-5a53-81fb-ac8b5f267d2c".to_string(),
        kind: "IMPORT".to_string(),
        processed_rows: Some(3),
        skipped: Some(3),
        status: "QUEUED".to_string(),
        total_rows: Some(3),
        updated: Some(3),
    })
}
