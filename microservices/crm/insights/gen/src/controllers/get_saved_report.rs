// User-owned controller for handler 'get_saved_report'.

use crate::handlers::get_saved_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSavedReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "data": {},
    //   "filters": {},
    //   "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    //   "name": "Northwind Traders",
    //   "type": "Type"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-11-05T16:00:00Z",
  "data": {},
  "filters": {},
  "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
  "name": "Northwind Traders",
  "type": "Type"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        data: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        filters: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        id: Some("aa70dbe4-37c6-567e-b827-49d9e6bcd18c".to_string()),
        name: "Northwind Traders".to_string(),
        r#type: "Type".to_string(),
    })
}
