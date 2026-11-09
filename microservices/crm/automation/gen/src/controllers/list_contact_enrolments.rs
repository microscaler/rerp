// User-owned controller for handler 'list_contact_enrolments'.

use crate::handlers::list_contact_enrolments::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Enrolment;

#[handler(ListContactEnrolmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_email": "priya.natarajan@northwindtraders.example",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "contact_name": "Priya Natarajan",
    //       "current_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
    //       "current_step_label": "Wait 2 days",
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
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "contact_email": "priya.natarajan@northwindtraders.example",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "contact_name": "Priya Natarajan",
      "current_step_id": "6987f07d-c099-529a-bb28-e900ce5e5d00",
      "current_step_label": "Wait 2 days",
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
    }
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<Enrolment>(serde_json::json!({"contact_email":"priya.natarajan@northwindtraders.example","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","current_step_id":"6987f07d-c099-529a-bb28-e900ce5e5d00","current_step_label":"Wait 2 days","ended_at":"2026-11-05T16:00:00Z","entered_at":"2026-11-03T16:00:00Z","entered_by":"alex","history":[{"at":"2026-11-03T16:00:00Z","kind":"ACTION","label":"Send email","outcome":"DONE","step_id":"f1189b89-3e27-5826-bc93-e33f7450d394"}],"id":"7579e087-fd2f-568e-8835-12a76256ad3e","next_run_at":"2026-11-06T16:00:00Z","status":"WAITING","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default()]))
}
