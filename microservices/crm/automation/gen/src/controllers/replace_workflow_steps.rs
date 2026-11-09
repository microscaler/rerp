// User-owned controller for handler 'replace_workflow_steps'.

use crate::handlers::replace_workflow_steps::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkflowStep;

#[handler(ReplaceWorkflowStepsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entry_step_id": "93404b0b-da88-5a3f-9bbc-46f4030b5dc2",
    //   "steps": [
    //     {
    //       "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //       "kind": "ACTION"
    //     }
    //   ],
    //   "version": 3,
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "entry_step_id": "93404b0b-da88-5a3f-9bbc-46f4030b5dc2",
  "steps": [
    {
      "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
      "kind": "ACTION"
    }
  ],
  "version": 3,
  "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        entry_step_id: "93404b0b-da88-5a3f-9bbc-46f4030b5dc2".to_string(),
        steps: vec![serde_json::from_value::<WorkflowStep>(
            serde_json::json!({"id":"c9e18116-9fea-5e6e-ac32-52580d2450b8","kind":"ACTION"}),
        )
        .unwrap_or_default()],
        version: Some(3),
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),
    })
}
