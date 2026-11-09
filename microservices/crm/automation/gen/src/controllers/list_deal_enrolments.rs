// User-owned controller for handler 'list_deal_enrolments'.

use crate::handlers::list_deal_enrolments::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Enrolment;

#[handler(ListDealEnrolmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "entered_at": "2026-11-01T16:00:00Z",
    //       "id": "47ac623d-96a0-57aa-b8c2-0b8a230023a6",
    //       "status": "COMPLETED",
    //       "workflow_id": "f1f1bd07-238b-520a-b007-7846dac00c19",
    //       "workflow_name": "Hot lead handoff"
    //     },
    //     {
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "current_step_label": "14 days no activity",
    //       "entered_at": "2026-11-02T16:00:00Z",
    //       "id": "3b209ba1-4c06-5235-a6e7-0c74f4d40366",
    //       "status": "WAITING",
    //       "workflow_id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496",
    //       "workflow_name": "Stalled deal nudge"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "entered_at": "2026-11-01T16:00:00Z",
      "id": "47ac623d-96a0-57aa-b8c2-0b8a230023a6",
      "status": "COMPLETED",
      "workflow_id": "f1f1bd07-238b-520a-b007-7846dac00c19",
      "workflow_name": "Hot lead handoff"
    },
    {
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "current_step_label": "14 days no activity",
      "entered_at": "2026-11-02T16:00:00Z",
      "id": "3b209ba1-4c06-5235-a6e7-0c74f4d40366",
      "status": "WAITING",
      "workflow_id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496",
      "workflow_name": "Stalled deal nudge"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Enrolment>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","entered_at":"2026-11-01T16:00:00Z","id":"47ac623d-96a0-57aa-b8c2-0b8a230023a6","status":"COMPLETED","workflow_id":"f1f1bd07-238b-520a-b007-7846dac00c19","workflow_name":"Hot lead handoff"})).unwrap_or_default(), serde_json::from_value::<Enrolment>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","current_step_label":"14 days no activity","entered_at":"2026-11-02T16:00:00Z","id":"3b209ba1-4c06-5235-a6e7-0c74f4d40366","status":"WAITING","workflow_id":"b36c0bc7-7dae-56b4-af7f-9c1ef4673496","workflow_name":"Stalled deal nudge"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
