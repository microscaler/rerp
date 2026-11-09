// User-owned controller for handler 'get_enrolment'.

use crate::handlers::get_enrolment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EnrolmentStatus;
#[allow(unused_imports)]
use crate::handlers::types::EnrolmentStepRecord;
#[allow(unused_imports)]
use crate::handlers::types::ExitReason;
#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(GetEnrolmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_email": "priya.natarajan@northwindtraders.example",
    //   "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "current_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
    //   "ended_at": "2026-11-05T16:00:00Z",
    //   "entered_at": "2026-11-03T16:00:00Z",
    //   "entered_by": "alex",
    //   "history": [
    //     {
    //       "at": "2026-11-03T16:00:00Z",
    //       "kind": "ACTION",
    //       "label": "Send email",
    //       "outcome": "DONE",
    //       "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
    //     }
    //   ],
    //   "id": "7579e087-fd2f-568e-8835-12a76256ad3e",
    //   "next_run_at": "2026-11-06T16:00:00Z",
    //   "status": "WAITING",
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //   "workflow_name": "Welcome series (7 days)"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contact_email": "priya.natarajan@northwindtraders.example",
  "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "current_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
  "ended_at": "2026-11-05T16:00:00Z",
  "entered_at": "2026-11-03T16:00:00Z",
  "entered_by": "alex",
  "history": [
    {
      "at": "2026-11-03T16:00:00Z",
      "kind": "ACTION",
      "label": "Send email",
      "outcome": "DONE",
      "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
    }
  ],
  "id": "7579e087-fd2f-568e-8835-12a76256ad3e",
  "next_run_at": "2026-11-06T16:00:00Z",
  "status": "WAITING",
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
  "workflow_name": "Welcome series (7 days)"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contact_email: Some("priya.natarajan@northwindtraders.example".to_string()),contact_id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(),current_step_id: Some("6987f07d-c099-529a-bb28-e900ce5e5d00".to_string()),ended_at: Some("2026-11-05T16:00:00Z".to_string()),entered_at: "2026-11-03T16:00:00Z".to_string(),entered_by: Some("alex".to_string()),exit_reason: Some(Default::default()),goal_step_id: Some("example".to_string()),history: Some(vec![serde_json::from_value::<EnrolmentStepRecord>(serde_json::json!({"at":"2026-11-03T16:00:00Z","kind":"ACTION","label":"Send email","outcome":"DONE","step_id":"f1189b89-3e27-5826-bc93-e33f7450d394"})).unwrap_or_default()]),id: "7579e087-fd2f-568e-8835-12a76256ad3e".to_string(),next_run_at: Some("2026-11-06T16:00:00Z".to_string()),split_paths: Some(Default::default()),status: serde_json::from_value::<EnrolmentStatus>(serde_json::Value::String("WAITING".to_string())).unwrap_or_default(),trigger_id: Some("example".to_string()),trigger_subject: Some("example".to_string()),trigger_type: Some(Default::default()),workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),workflow_name: Some("Welcome series (7 days)".to_string()),
    })
}
