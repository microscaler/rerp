// User-owned controller for handler 'create_data_request'.

use crate::handlers::create_data_request::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDataRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_email": "marcus.reid@adventure.example",
    //   "created_at": "2026-11-02T16:00:00Z",
    //   "id": "c4faeb88-b400-5f86-84ae-47f52514f082",
    //   "kind": "ACCESS",
    //   "note": "2.1 MB JSON",
    //   "requested_by": "Contact · preference page",
    //   "status": "DONE"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contact_email": "marcus.reid@adventure.example",
  "created_at": "2026-11-02T16:00:00Z",
  "id": "c4faeb88-b400-5f86-84ae-47f52514f082",
  "kind": "ACCESS",
  "note": "2.1 MB JSON",
  "requested_by": "Contact · preference page",
  "status": "DONE"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        completed_at: Some("example".to_string()),
        contact_email: Some("marcus.reid@adventure.example".to_string()),
        created_at: "2026-11-02T16:00:00Z".to_string(),
        id: "c4faeb88-b400-5f86-84ae-47f52514f082".to_string(),
        kind: "ACCESS".to_string(),
        note: Some("2.1 MB JSON".to_string()),
        requested_by: Some("Contact · preference page".to_string()),
        source: Some("example".to_string()),
        status: "DONE".to_string(),
        steps: Some(vec![]),
        subject_hash: Some("example".to_string()),
    })
}
