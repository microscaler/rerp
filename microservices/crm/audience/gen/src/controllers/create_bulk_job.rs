// User-owned controller for handler 'create_bulk_job'.

use crate::handlers::create_bulk_job::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BulkJobAction;

#[handler(CreateBulkJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "action": "ADD_TAG",
    //   "by": "Riya Kapoor",
    //   "contacts": 418,
    //   "created_at": "2026-10-23T16:00:00Z",
    //   "id": "c16c46aa-99a8-54ba-a51c-952111d137c1",
    //   "label": "Add tag webinar-oct",
    //   "status": "DONE",
    //   "undoable": true,
    //   "value": "webinar-oct"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "action": "ADD_TAG",
  "by": "Riya Kapoor",
  "contacts": 418,
  "created_at": "2026-10-23T16:00:00Z",
  "id": "c16c46aa-99a8-54ba-a51c-952111d137c1",
  "label": "Add tag webinar-oct",
  "status": "DONE",
  "undoable": true,
  "value": "webinar-oct"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        action: serde_json::from_value::<BulkJobAction>(serde_json::Value::String(
            "ADD_TAG".to_string(),
        ))
        .unwrap_or_default(),
        by: Some("Riya Kapoor".to_string()),
        contacts: Some(418),
        created_at: "2026-10-23T16:00:00Z".to_string(),
        field_key: Some("example".to_string()),
        finished_at: Some("example".to_string()),
        id: "c16c46aa-99a8-54ba-a51c-952111d137c1".to_string(),
        label: Some("Add tag webinar-oct".to_string()),
        list_id: Some("example".to_string()),
        segment_id: Some("example".to_string()),
        status: "DONE".to_string(),
        undo_until: Some("example".to_string()),
        undoable: Some(true),
        value: Some("webinar-oct".to_string()),
    })
}
