// User-owned controller for handler 'get_workflow_map'.

use crate::handlers::get_workflow_map::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWorkflowMapController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": 1204,
    //   "completed": 8311,
    //   "goal_reached": 1911,
    //   "steps": [
    //     {
    //       "branch_no": 0,
    //       "branch_yes": 0,
    //       "clicked": 1142,
    //       "failed": 2,
    //       "kind": "ACTION",
    //       "label": "Send email",
    //       "opened": 5899,
    //       "passed": 9515,
    //       "sent": 9515,
    //       "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //       "waiting_now": 0
    //     },
    //     {
    //       "branch_no": 0,
    //       "branch_yes": 0,
    //       "clicked": 0,
    //       "failed": 0,
    //       "kind": "WAIT",
    //       "label": "Wait 2 days",
    //       "opened": 0,
    //       "passed": 9513,
    //       "sent": 0,
    //       "step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
    //       "waiting_now": 402
    //     },
    //     {
    //       "branch_no": 3471,
    //       "branch_yes": 5640,
    //       "clicked": 0,
    //       "failed": 0,
    //       "kind": "IF_ELSE",
    //       "label": "Opened the previous email?",
    //       "opened": 0,
    //       "passed": 9111,
    //       "sent": 0,
    //       "step_id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
    //       "waiting_now": 0
    //     }
    //   ],
    //   "stopped": 89,
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "active": 1204,
  "completed": 8311,
  "goal_reached": 1911,
  "steps": [
    {
      "branch_no": 0,
      "branch_yes": 0,
      "clicked": 1142,
      "failed": 2,
      "kind": "ACTION",
      "label": "Send email",
      "opened": 5899,
      "passed": 9515,
      "sent": 9515,
      "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
      "waiting_now": 0
    },
    {
      "branch_no": 0,
      "branch_yes": 0,
      "clicked": 0,
      "failed": 0,
      "kind": "WAIT",
      "label": "Wait 2 days",
      "opened": 0,
      "passed": 9513,
      "sent": 0,
      "step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
      "waiting_now": 402
    },
    {
      "branch_no": 3471,
      "branch_yes": 5640,
      "clicked": 0,
      "failed": 0,
      "kind": "IF_ELSE",
      "label": "Opened the previous email?",
      "opened": 0,
      "passed": 9111,
      "sent": 0,
      "step_id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
      "waiting_now": 0
    }
  ],
  "stopped": 89,
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: 1204,completed: 8311,goal_reached: Some(1911),steps: vec![serde_json::from_str::<serde_json::Value>(r#"{"branch_no":0,"branch_yes":0,"clicked":1142,"failed":2,"kind":"ACTION","label":"Send email","opened":5899,"passed":9515,"sent":9515,"step_id":"f1189b89-3e27-5826-bc93-e33f7450d394","waiting_now":0}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"branch_no":0,"branch_yes":0,"clicked":0,"failed":0,"kind":"WAIT","label":"Wait 2 days","opened":0,"passed":9513,"sent":0,"step_id":"6987f07d-c099-529a-bb28-e900ce5e5d00","waiting_now":402}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"branch_no":3471,"branch_yes":5640,"clicked":0,"failed":0,"kind":"IF_ELSE","label":"Opened the previous email?","opened":0,"passed":9111,"sent":0,"step_id":"94f0ecb3-8d57-54c0-937b-10c11b11cb2c","waiting_now":0}"#).unwrap_or(serde_json::Value::Null)],stopped: Some(89),workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
    })
}
