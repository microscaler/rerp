// User-owned controller for handler 'retry_enrolment'.

use crate::handlers::retry_enrolment::{Request, Response};
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

#[handler(RetryEnrolmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_email": "Contact email",
    //   "contact_id": "7970f5e9-e936-5cd5-b2cf-883a6f1433fc",
    //   "current_step_id": "e73c31c1-5129-50f1-ad4a-d05281d2a22a",
    //   "ended_at": "2026-11-05T16:00:00Z",
    //   "entered_at": "2026-11-05T16:00:00Z",
    //   "entered_by": "Entered by",
    //   "exit_reason": "UNSUBSCRIBED",
    //   "goal_step_id": "e9a62b94-9bdb-56da-9a42-0eacb6e57d28",
    //   "history": [
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "kind": "ACTION",
    //       "outcome": "DONE",
    //       "step_id": "cd9627a9-c1de-5aef-9be3-a38c1d7460bd"
    //     }
    //   ],
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "next_run_at": "2026-11-05T16:00:00Z",
    //   "split_paths": {},
    //   "status": "ACTIVE",
    //   "trigger_id": "848ded80-ea6f-57b1-a733-796a8102a816",
    //   "trigger_subject": "Trigger subject",
    //   "trigger_type": "STAGE_CHANGE",
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548",
    //   "workflow_name": "Workflow name"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "contact_email": "Contact email",
  "contact_id": "7970f5e9-e936-5cd5-b2cf-883a6f1433fc",
  "current_step_id": "e73c31c1-5129-50f1-ad4a-d05281d2a22a",
  "ended_at": "2026-11-05T16:00:00Z",
  "entered_at": "2026-11-05T16:00:00Z",
  "entered_by": "Entered by",
  "exit_reason": "UNSUBSCRIBED",
  "goal_step_id": "e9a62b94-9bdb-56da-9a42-0eacb6e57d28",
  "history": [
    {
      "at": "2026-11-05T16:00:00Z",
      "kind": "ACTION",
      "outcome": "DONE",
      "step_id": "cd9627a9-c1de-5aef-9be3-a38c1d7460bd"
    }
  ],
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "next_run_at": "2026-11-05T16:00:00Z",
  "split_paths": {},
  "status": "ACTIVE",
  "trigger_id": "848ded80-ea6f-57b1-a733-796a8102a816",
  "trigger_subject": "Trigger subject",
  "trigger_type": "STAGE_CHANGE",
  "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548",
  "workflow_name": "Workflow name"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contact_email: Some("Contact email".to_string()),contact_id: "7970f5e9-e936-5cd5-b2cf-883a6f1433fc".to_string(),current_step_id: Some("e73c31c1-5129-50f1-ad4a-d05281d2a22a".to_string()),ended_at: Some("2026-11-05T16:00:00Z".to_string()),entered_at: "2026-11-05T16:00:00Z".to_string(),entered_by: Some("Entered by".to_string()),exit_reason: Some(serde_json::from_value::<ExitReason>(serde_json::Value::String("UNSUBSCRIBED".to_string())).unwrap_or_default()),goal_step_id: Some("e9a62b94-9bdb-56da-9a42-0eacb6e57d28".to_string()),history: Some(vec![serde_json::from_value::<EnrolmentStepRecord>(serde_json::json!({"at":"2026-11-05T16:00:00Z","kind":"ACTION","outcome":"DONE","step_id":"cd9627a9-c1de-5aef-9be3-a38c1d7460bd"})).unwrap_or_default()]),id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),next_run_at: Some("2026-11-05T16:00:00Z".to_string()),split_paths: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),status: serde_json::from_value::<EnrolmentStatus>(serde_json::Value::String("ACTIVE".to_string())).unwrap_or_default(),trigger_id: Some("848ded80-ea6f-57b1-a733-796a8102a816".to_string()),trigger_subject: Some("Trigger subject".to_string()),trigger_type: Some(serde_json::from_value::<TriggerType>(serde_json::Value::String("STAGE_CHANGE".to_string())).unwrap_or_default()),workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),workflow_name: Some("Workflow name".to_string()),
    })
}
