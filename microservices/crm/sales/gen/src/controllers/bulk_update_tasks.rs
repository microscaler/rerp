// User-owned controller for handler 'bulk_update_tasks'.

use crate::handlers::bulk_update_tasks::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BulkUpdateTasksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "skipped": [
    //     {}
    //   ],
    //   "updated": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "skipped": [
    {}
  ],
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
        skipped: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        updated: Some(3),
    })
}
