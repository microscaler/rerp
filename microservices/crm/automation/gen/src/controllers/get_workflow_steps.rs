// User-owned controller for handler 'get_workflow_steps'.

use crate::handlers::get_workflow_steps::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkflowStep;

#[handler(GetWorkflowStepsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //   "steps": [
    //     {
    //       "action_config": {
    //         "template_id": "736685cb-324b-572c-b287-deb656a4d8de",
    //         "template_name": "Welcome · day 0: what you can do today"
    //       },
    //       "action_type": "SEND_MESSAGE",
    //       "id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //       "kind": "ACTION",
    //       "label": "Send email",
    //       "next_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00"
    //     },
    //     {
    //       "id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
    //       "kind": "WAIT",
    //       "label": "Wait 2 days",
    //       "next_step_id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
    //       "wait": {
    //         "mode": "DURATION",
    //         "seconds": 172800,
    //         "send_window": {
    //           "days": [
    //             "MON",
    //             "TUE",
    //             "WED"
    //           ],
    //           "from_hour": 8,
    //           "to_hour": 18
    //         }
    //       }
    //     },
    //     {
    //       "conditions": {
    //         "items": [
    //           {
    //             "operator": "HAS",
    //             "subject": "engagement:opened:2"
    //           }
    //         ],
    //         "op": "AND"
    //       },
    //       "id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
    //       "kind": "IF_ELSE",
    //       "label": "Opened the previous email?",
    //       "no_step_id": "05f191ea-4671-5e2e-b6d2-4ac551d5d22c",
    //       "yes_step_id": "27989e3e-01b9-5f9d-b481-ca400c08298b"
    //     }
    //   ],
    //   "version": 7,
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
  "steps": [
    {
      "action_config": {
        "template_id": "736685cb-324b-572c-b287-deb656a4d8de",
        "template_name": "Welcome · day 0: what you can do today"
      },
      "action_type": "SEND_MESSAGE",
      "id": "f1189b89-3e27-5826-bc93-e33f7450d394",
      "kind": "ACTION",
      "label": "Send email",
      "next_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00"
    },
    {
      "id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
      "kind": "WAIT",
      "label": "Wait 2 days",
      "next_step_id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
      "wait": {
        "mode": "DURATION",
        "seconds": 172800,
        "send_window": {
          "days": [
            "MON",
            "TUE",
            "WED"
          ],
          "from_hour": 8,
          "to_hour": 18
        }
      }
    },
    {
      "conditions": {
        "items": [
          {
            "operator": "HAS",
            "subject": "engagement:opened:2"
          }
        ],
        "op": "AND"
      },
      "id": "94f0ecb3-8d57-54c0-937b-10c11b11cb2c",
      "kind": "IF_ELSE",
      "label": "Opened the previous email?",
      "no_step_id": "05f191ea-4671-5e2e-b6d2-4ac551d5d22c",
      "yes_step_id": "27989e3e-01b9-5f9d-b481-ca400c08298b"
    }
  ],
  "version": 7,
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
        entry_step_id: "f1189b89-3e27-5826-bc93-e33f7450d394".to_string(),steps: vec![serde_json::from_value::<WorkflowStep>(serde_json::json!({"action_config":{"template_id":"736685cb-324b-572c-b287-deb656a4d8de","template_name":"Welcome · day 0: what you can do today"},"action_type":"SEND_MESSAGE","id":"f1189b89-3e27-5826-bc93-e33f7450d394","kind":"ACTION","label":"Send email","next_step_id":"6987f07d-c099-529a-bb28-e900ce5e5d00"})).unwrap_or_default(), serde_json::from_value::<WorkflowStep>(serde_json::json!({"id":"6987f07d-c099-529a-bb28-e900ce5e5d00","kind":"WAIT","label":"Wait 2 days","next_step_id":"94f0ecb3-8d57-54c0-937b-10c11b11cb2c","wait":{"mode":"DURATION","seconds":172800,"send_window":{"days":["MON","TUE","WED"],"from_hour":8,"to_hour":18}}})).unwrap_or_default(), serde_json::from_value::<WorkflowStep>(serde_json::json!({"conditions":{"items":[{"operator":"HAS","subject":"engagement:opened:2"}],"op":"AND"},"id":"94f0ecb3-8d57-54c0-937b-10c11b11cb2c","kind":"IF_ELSE","label":"Opened the previous email?","no_step_id":"05f191ea-4671-5e2e-b6d2-4ac551d5d22c","yes_step_id":"27989e3e-01b9-5f9d-b481-ca400c08298b"})).unwrap_or_default()],version: Some(7),workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
    })
}
