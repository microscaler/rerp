// User-owned controller for handler 'list_data_requests'.

use crate::handlers::list_data_requests::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DataRequest;

#[handler(ListDataRequestsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_email": "marcus.reid@adventure.example",
    //       "created_at": "2026-11-02T16:00:00Z",
    //       "id": "c4faeb88-b400-5f86-84ae-47f52514f082",
    //       "kind": "ACCESS",
    //       "note": "2.1 MB JSON",
    //       "requested_by": "Contact · preference page",
    //       "status": "DONE"
    //     },
    //     {
    //       "contact_email": "old-team@fabrikam.example",
    //       "created_at": "2026-09-30T16:00:00Z",
    //       "id": "bac118f7-c5eb-539d-bde0-41b5b513bce3",
    //       "kind": "ACCESS",
    //       "note": "Anonymised · consent log kept",
    //       "requested_by": "Alex Morgan · ticket #4412",
    //       "status": "DONE"
    //     },
    //     {
    //       "contact_email": "j.doe@example.org",
    //       "created_at": "2026-11-04T16:00:00Z",
    //       "id": "8b40f48a-b29b-5329-a1e1-39f7966192c4",
    //       "kind": "ACCESS",
    //       "note": "28 days left",
    //       "requested_by": "Contact · email to privacy@",
    //       "status": "PENDING_REVIEW"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "contact_email": "marcus.reid@adventure.example",
      "created_at": "2026-11-02T16:00:00Z",
      "id": "c4faeb88-b400-5f86-84ae-47f52514f082",
      "kind": "ACCESS",
      "note": "2.1 MB JSON",
      "requested_by": "Contact · preference page",
      "status": "DONE"
    },
    {
      "contact_email": "old-team@fabrikam.example",
      "created_at": "2026-09-30T16:00:00Z",
      "id": "bac118f7-c5eb-539d-bde0-41b5b513bce3",
      "kind": "ACCESS",
      "note": "Anonymised · consent log kept",
      "requested_by": "Alex Morgan · ticket #4412",
      "status": "DONE"
    },
    {
      "contact_email": "j.doe@example.org",
      "created_at": "2026-11-04T16:00:00Z",
      "id": "8b40f48a-b29b-5329-a1e1-39f7966192c4",
      "kind": "ACCESS",
      "note": "28 days left",
      "requested_by": "Contact · email to privacy@",
      "status": "PENDING_REVIEW"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"marcus.reid@adventure.example","created_at":"2026-11-02T16:00:00Z","id":"c4faeb88-b400-5f86-84ae-47f52514f082","kind":"ACCESS","note":"2.1 MB JSON","requested_by":"Contact · preference page","status":"DONE"})).unwrap_or_default(), serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"old-team@fabrikam.example","created_at":"2026-09-30T16:00:00Z","id":"bac118f7-c5eb-539d-bde0-41b5b513bce3","kind":"ACCESS","note":"Anonymised · consent log kept","requested_by":"Alex Morgan · ticket #4412","status":"DONE"})).unwrap_or_default(), serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"j.doe@example.org","created_at":"2026-11-04T16:00:00Z","id":"8b40f48a-b29b-5329-a1e1-39f7966192c4","kind":"ACCESS","note":"28 days left","requested_by":"Contact · email to privacy@","status":"PENDING_REVIEW"})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
