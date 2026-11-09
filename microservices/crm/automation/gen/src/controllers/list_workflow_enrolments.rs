// User-owned controller for handler 'list_workflow_enrolments'.

use crate::handlers::list_workflow_enrolments::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Enrolment;

#[handler(ListWorkflowEnrolmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_email": "priya.natarajan@northwindtraders.example",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "current_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
    //       "ended_at": "2026-11-05T16:00:00Z",
    //       "entered_at": "2026-11-03T16:00:00Z",
    //       "entered_by": "alex",
    //       "history": [
    //         {
    //           "at": "2026-11-03T16:00:00Z",
    //           "kind": "ACTION",
    //           "label": "Send email",
    //           "outcome": "DONE",
    //           "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
    //         }
    //       ],
    //       "id": "7579e087-fd2f-568e-8835-12a76256ad3e",
    //       "next_run_at": "2026-11-06T16:00:00Z",
    //       "status": "WAITING",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     },
    //     {
    //       "contact_email": "tom.okafor@fabrikam.example",
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "current_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //       "ended_at": "2026-11-05T16:00:00Z",
    //       "entered_at": "2026-11-02T16:00:00Z",
    //       "entered_by": "TRIGGER",
    //       "history": [
    //         {
    //           "at": "2026-11-02T16:00:00Z",
    //           "kind": "ACTION",
    //           "label": "Send email",
    //           "outcome": "DONE",
    //           "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
    //         }
    //       ],
    //       "id": "4a88513a-fb6e-56c4-a4d0-7ec3692b5d6d",
    //       "next_run_at": "2026-11-05T16:00:00Z",
    //       "status": "ACTIVE",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     },
    //     {
    //       "contact_email": "lena.fischer@contoso.example",
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "current_step_id": "ef423ff6-5584-5017-b1b3-8408d7958b9e",
    //       "ended_at": "2026-11-05T16:00:00Z",
    //       "entered_at": "2026-11-01T16:00:00Z",
    //       "entered_by": "TRIGGER",
    //       "history": [
    //         {
    //           "at": "2026-11-01T16:00:00Z",
    //           "kind": "ACTION",
    //           "label": "Send email",
    //           "outcome": "DONE",
    //           "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
    //         }
    //       ],
    //       "id": "af9f8ad1-8b8e-5dab-8831-9882125ee623",
    //       "next_run_at": "2026-11-08T16:00:00Z",
    //       "status": "WAITING",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     }
    //   ],
    //   "limit": 100,
    //   "page": 1,
    //   "total": 24
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
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
    },
    {
      "contact_email": "tom.okafor@fabrikam.example",
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "current_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
      "ended_at": "2026-11-05T16:00:00Z",
      "entered_at": "2026-11-02T16:00:00Z",
      "entered_by": "TRIGGER",
      "history": [
        {
          "at": "2026-11-02T16:00:00Z",
          "kind": "ACTION",
          "label": "Send email",
          "outcome": "DONE",
          "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
        }
      ],
      "id": "4a88513a-fb6e-56c4-a4d0-7ec3692b5d6d",
      "next_run_at": "2026-11-05T16:00:00Z",
      "status": "ACTIVE",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    },
    {
      "contact_email": "lena.fischer@contoso.example",
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "current_step_id": "ef423ff6-5584-5017-b1b3-8408d7958b9e",
      "ended_at": "2026-11-05T16:00:00Z",
      "entered_at": "2026-11-01T16:00:00Z",
      "entered_by": "TRIGGER",
      "history": [
        {
          "at": "2026-11-01T16:00:00Z",
          "kind": "ACTION",
          "label": "Send email",
          "outcome": "DONE",
          "step_id": "f1189b89-3e27-5826-bc93-e33f7450d394"
        }
      ],
      "id": "af9f8ad1-8b8e-5dab-8831-9882125ee623",
      "next_run_at": "2026-11-08T16:00:00Z",
      "status": "WAITING",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    }
  ],
  "limit": 100,
  "page": 1,
  "total": 24
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Enrolment>(serde_json::json!({"contact_email":"priya.natarajan@northwindtraders.example","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","current_step_id":"6987f07d-c099-529a-bb28-e900ce5e5d00","ended_at":"2026-11-05T16:00:00Z","entered_at":"2026-11-03T16:00:00Z","entered_by":"alex","history":[{"at":"2026-11-03T16:00:00Z","kind":"ACTION","label":"Send email","outcome":"DONE","step_id":"f1189b89-3e27-5826-bc93-e33f7450d394"}],"id":"7579e087-fd2f-568e-8835-12a76256ad3e","next_run_at":"2026-11-06T16:00:00Z","status":"WAITING","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default(), serde_json::from_value::<Enrolment>(serde_json::json!({"contact_email":"tom.okafor@fabrikam.example","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","current_step_id":"f1189b89-3e27-5826-bc93-e33f7450d394","ended_at":"2026-11-05T16:00:00Z","entered_at":"2026-11-02T16:00:00Z","entered_by":"TRIGGER","history":[{"at":"2026-11-02T16:00:00Z","kind":"ACTION","label":"Send email","outcome":"DONE","step_id":"f1189b89-3e27-5826-bc93-e33f7450d394"}],"id":"4a88513a-fb6e-56c4-a4d0-7ec3692b5d6d","next_run_at":"2026-11-05T16:00:00Z","status":"ACTIVE","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default(), serde_json::from_value::<Enrolment>(serde_json::json!({"contact_email":"lena.fischer@contoso.example","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","current_step_id":"ef423ff6-5584-5017-b1b3-8408d7958b9e","ended_at":"2026-11-05T16:00:00Z","entered_at":"2026-11-01T16:00:00Z","entered_by":"TRIGGER","history":[{"at":"2026-11-01T16:00:00Z","kind":"ACTION","label":"Send email","outcome":"DONE","step_id":"f1189b89-3e27-5826-bc93-e33f7450d394"}],"id":"af9f8ad1-8b8e-5dab-8831-9882125ee623","next_run_at":"2026-11-08T16:00:00Z","status":"WAITING","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default()],limit: 100,page: 1,total: 24,
    })
}
