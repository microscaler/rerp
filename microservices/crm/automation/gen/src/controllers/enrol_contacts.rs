// User-owned controller for handler 'enrol_contacts'.

use crate::handlers::enrol_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(EnrolContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "already_active": 3,
    //   "enrolled": 0,
    //   "excluded": 3,
    //   "skipped": [
    //     {}
    //   ],
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "already_active": 3,
  "enrolled": 0,
  "excluded": 3,
  "skipped": [
    {}
  ],
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        already_active: 3,
        enrolled: 0,
        enrolment_ids: Some(vec![]),
        excluded: 3,
        skipped: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        workflow_id: Some("22e49987-c27c-538a-9688-56d9ca17a7ec".to_string()),
    })
}
