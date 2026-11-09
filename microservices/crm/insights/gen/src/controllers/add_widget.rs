// User-owned controller for handler 'add_widget'.

use crate::handlers::add_widget::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReportSchedule;
#[allow(unused_imports)]
use crate::handlers::types::WidgetQuery;

#[handler(AddWidgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "3d0b28d9-5ddf-5113-8e16-97b0bc878ccf",
    //   "kind": "TABLE",
    //   "position": 5,
    //   "query": {
    //     "dimension": "list",
    //     "metric": "list_count",
    //     "range": "LAST_30_DAYS"
    //   },
    //   "refreshed_at": "2026-11-05T16:00:00Z",
    //   "title": "Contacts per list",
    //   "width": 1
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "id": "3d0b28d9-5ddf-5113-8e16-97b0bc878ccf",
  "kind": "TABLE",
  "position": 5,
  "query": {
    "dimension": "list",
    "metric": "list_count",
    "range": "LAST_30_DAYS"
  },
  "refreshed_at": "2026-11-05T16:00:00Z",
  "title": "Contacts per list",
  "width": 1
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        id: "3d0b28d9-5ddf-5113-8e16-97b0bc878ccf".to_string(),
        kind: "TABLE".to_string(),
        position: Some(5),
        prompt: Some("example".to_string()),
        query: serde_json::from_value::<WidgetQuery>(
            serde_json::json!({"dimension":"list","metric":"list_count","range":"LAST_30_DAYS"}),
        )
        .unwrap_or_default(),
        refreshed_at: Some("2026-11-05T16:00:00Z".to_string()),
        schedule: Some(Default::default()),
        title: "Contacts per list".to_string(),
        width: Some(1),
    })
}
