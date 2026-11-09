// User-owned controller for handler 'get_contact_activity'.

use crate::handlers::get_contact_activity::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Activity;

#[handler(GetContactActivityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "actor": "contact",
    //       "at": "2026-11-05T14:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "detail": "Campaign · opened twice, clicked Read the changelog",
    //       "id": "ec537d04-feb3-5edb-8713-78bcb173e477",
    //       "kind": "MESSAGE_OPENED",
    //       "summary": "Opened \"October product update\""
    //     },
    //     {
    //       "actor": "contact",
    //       "at": "2026-11-05T11:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "detail": "From email link · 2 min 40 s · also /docs/sso",
    //       "id": "17b94f6b-e922-5fd5-8c50-7e7f4a9483f7",
    //       "kind": "PAGE_VIEW",
    //       "summary": "Visited /pricing"
    //     },
    //     {
    //       "actor": "contact",
    //       "at": "2026-11-04T16:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "detail": "Trigger: date field renewal_date − 30 days",
    //       "id": "85b3de6c-cd6e-5c99-a045-33d7479d3f12",
    //       "kind": "WORKFLOW_ENTERED",
    //       "summary": "Entered \"Renewal reminder · 30 days out\""
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 10
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "actor": "contact",
      "at": "2026-11-05T14:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "detail": "Campaign · opened twice, clicked Read the changelog",
      "id": "ec537d04-feb3-5edb-8713-78bcb173e477",
      "kind": "MESSAGE_OPENED",
      "summary": "Opened \"October product update\""
    },
    {
      "actor": "contact",
      "at": "2026-11-05T11:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "detail": "From email link · 2 min 40 s · also /docs/sso",
      "id": "17b94f6b-e922-5fd5-8c50-7e7f4a9483f7",
      "kind": "PAGE_VIEW",
      "summary": "Visited /pricing"
    },
    {
      "actor": "contact",
      "at": "2026-11-04T16:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "detail": "Trigger: date field renewal_date − 30 days",
      "id": "85b3de6c-cd6e-5c99-a045-33d7479d3f12",
      "kind": "WORKFLOW_ENTERED",
      "summary": "Entered \"Renewal reminder · 30 days out\""
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 10
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-05T14:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"Campaign · opened twice, clicked Read the changelog","id":"ec537d04-feb3-5edb-8713-78bcb173e477","kind":"MESSAGE_OPENED","summary":"Opened \"October product update\""})).unwrap_or_default(), serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-05T11:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"From email link · 2 min 40 s · also /docs/sso","id":"17b94f6b-e922-5fd5-8c50-7e7f4a9483f7","kind":"PAGE_VIEW","summary":"Visited /pricing"})).unwrap_or_default(), serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-04T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"Trigger: date field renewal_date − 30 days","id":"85b3de6c-cd6e-5c99-a045-33d7479d3f12","kind":"WORKFLOW_ENTERED","summary":"Entered \"Renewal reminder · 30 days out\""})).unwrap_or_default()],limit: 20,page: 1,total: 10,
    })
}
