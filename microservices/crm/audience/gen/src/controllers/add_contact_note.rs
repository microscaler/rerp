// User-owned controller for handler 'add_contact_note'.

use crate::handlers::add_contact_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActivityKind;

#[handler(AddContactNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "at": "2026-11-05T16:00:00Z",
    //   "contact_id": "07a7b393-a2b8-5f9d-86c2-37bc92d15398",
    //   "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "kind": "MESSAGE_SENT",
    //   "summary": "Opened \"October product update\""
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "at": "2026-11-05T16:00:00Z",
  "contact_id": "07a7b393-a2b8-5f9d-86c2-37bc92d15398",
  "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "kind": "MESSAGE_SENT",
  "summary": "Opened \"October product update\""
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        actor: Some("example".to_string()),
        at: "2026-11-05T16:00:00Z".to_string(),
        contact_id: "07a7b393-a2b8-5f9d-86c2-37bc92d15398".to_string(),
        data: Some(Default::default()),
        detail: Some("example".to_string()),
        id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(),
        kind: serde_json::from_value::<ActivityKind>(serde_json::Value::String(
            "MESSAGE_SENT".to_string(),
        ))
        .unwrap_or_default(),
        ref_id: Some("example".to_string()),
        ref_service: Some("example".to_string()),
        summary: "Opened \"October product update\"".to_string(),
    })
}
