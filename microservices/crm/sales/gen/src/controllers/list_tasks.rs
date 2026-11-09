// User-owned controller for handler 'list_tasks'.

use crate::handlers::list_tasks::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Task;

#[handler(ListTasksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "completed_at": "2026-11-05T16:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "contact_name": "Priya Natarajan",
    //       "created_at": "2026-10-31T16:00:00Z",
    //       "created_by": "alex",
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "deal_title": "Northwind · 48 → 120 seats",
    //       "due_at": "2026-11-05T16:00:00Z",
    //       "id": "89a25b3e-8a9c-514a-b373-cd9ef34c7cbc",
    //       "outcome": "Outcome",
    //       "overdue": true,
    //       "owner_name": "Alex Morgan",
    //       "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
    //       "related_kind": "CONTACT",
    //       "status": "OPEN",
    //       "title": "Call about SSO requirements",
    //       "type": "Call",
    //       "type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
    //     },
    //     {
    //       "completed_at": "2026-11-05T16:00:00Z",
    //       "contact_id": "6db34cb3-bbbf-5e3f-9502-4efd0ee61e8e",
    //       "contact_name": "Sofia Álvarez",
    //       "created_at": "2026-10-30T16:00:00Z",
    //       "created_by": "automation:hot-lead",
    //       "deal_id": "f8691375-235e-50ea-b9f2-8fc4121d6435",
    //       "deal_title": "Tailspin · 25 seats",
    //       "due_at": "2026-11-05T16:00:00Z",
    //       "id": "9b417a13-8ac2-50ed-84aa-3048a5f953f7",
    //       "outcome": "Outcome",
    //       "overdue": true,
    //       "owner_name": "Alex Morgan",
    //       "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
    //       "related_kind": "CONTACT",
    //       "status": "OPEN",
    //       "title": "Send pricing proposal",
    //       "type": "Email",
    //       "type_id": "099104fc-dcee-57bf-b906-d2b826beb51d"
    //     },
    //     {
    //       "completed_at": "2026-11-05T16:00:00Z",
    //       "contact_id": "b7ad6bb7-2c27-526b-8340-a18d60fb176b",
    //       "contact_name": "Oliver Brown",
    //       "created_at": "2026-10-29T16:00:00Z",
    //       "created_by": "sam",
    //       "deal_id": "c50637e3-5f8b-5d72-bb89-8fe81b9eb36f",
    //       "deal_title": "Alpine Ski · migration",
    //       "due_at": "2026-11-06T16:00:00Z",
    //       "id": "a747276f-e470-5c47-9a27-1129890e7282",
    //       "outcome": "Outcome",
    //       "overdue": false,
    //       "owner_name": "Sam Lee",
    //       "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
    //       "related_kind": "CONTACT",
    //       "status": "OPEN",
    //       "title": "Demo",
    //       "type": "Meeting",
    //       "type_id": "645bfe17-69c3-558e-82cd-9594c3609fa2"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 8
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "completed_at": "2026-11-05T16:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "contact_name": "Priya Natarajan",
      "created_at": "2026-10-31T16:00:00Z",
      "created_by": "alex",
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "deal_title": "Northwind · 48 → 120 seats",
      "due_at": "2026-11-05T16:00:00Z",
      "id": "89a25b3e-8a9c-514a-b373-cd9ef34c7cbc",
      "outcome": "Outcome",
      "overdue": true,
      "owner_name": "Alex Morgan",
      "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
      "related_kind": "CONTACT",
      "status": "OPEN",
      "title": "Call about SSO requirements",
      "type": "Call",
      "type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
    },
    {
      "completed_at": "2026-11-05T16:00:00Z",
      "contact_id": "6db34cb3-bbbf-5e3f-9502-4efd0ee61e8e",
      "contact_name": "Sofia Álvarez",
      "created_at": "2026-10-30T16:00:00Z",
      "created_by": "automation:hot-lead",
      "deal_id": "f8691375-235e-50ea-b9f2-8fc4121d6435",
      "deal_title": "Tailspin · 25 seats",
      "due_at": "2026-11-05T16:00:00Z",
      "id": "9b417a13-8ac2-50ed-84aa-3048a5f953f7",
      "outcome": "Outcome",
      "overdue": true,
      "owner_name": "Alex Morgan",
      "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
      "related_kind": "CONTACT",
      "status": "OPEN",
      "title": "Send pricing proposal",
      "type": "Email",
      "type_id": "099104fc-dcee-57bf-b906-d2b826beb51d"
    },
    {
      "completed_at": "2026-11-05T16:00:00Z",
      "contact_id": "b7ad6bb7-2c27-526b-8340-a18d60fb176b",
      "contact_name": "Oliver Brown",
      "created_at": "2026-10-29T16:00:00Z",
      "created_by": "sam",
      "deal_id": "c50637e3-5f8b-5d72-bb89-8fe81b9eb36f",
      "deal_title": "Alpine Ski · migration",
      "due_at": "2026-11-06T16:00:00Z",
      "id": "a747276f-e470-5c47-9a27-1129890e7282",
      "outcome": "Outcome",
      "overdue": false,
      "owner_name": "Sam Lee",
      "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
      "related_kind": "CONTACT",
      "status": "OPEN",
      "title": "Demo",
      "type": "Meeting",
      "type_id": "645bfe17-69c3-558e-82cd-9594c3609fa2"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 8
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Task>(serde_json::json!({"completed_at":"2026-11-05T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","created_at":"2026-10-31T16:00:00Z","created_by":"alex","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","deal_title":"Northwind · 48 → 120 seats","due_at":"2026-11-05T16:00:00Z","id":"89a25b3e-8a9c-514a-b373-cd9ef34c7cbc","outcome":"Outcome","overdue":true,"owner_name":"Alex Morgan","related_id":"57d59745-33ec-5dc8-9adc-cc139255687c","related_kind":"CONTACT","status":"OPEN","title":"Call about SSO requirements","type":"Call","type_id":"4eec3526-5869-5e76-820f-ac2358c6b899"})).unwrap_or_default(), serde_json::from_value::<Task>(serde_json::json!({"completed_at":"2026-11-05T16:00:00Z","contact_id":"6db34cb3-bbbf-5e3f-9502-4efd0ee61e8e","contact_name":"Sofia Álvarez","created_at":"2026-10-30T16:00:00Z","created_by":"automation:hot-lead","deal_id":"f8691375-235e-50ea-b9f2-8fc4121d6435","deal_title":"Tailspin · 25 seats","due_at":"2026-11-05T16:00:00Z","id":"9b417a13-8ac2-50ed-84aa-3048a5f953f7","outcome":"Outcome","overdue":true,"owner_name":"Alex Morgan","related_id":"57d59745-33ec-5dc8-9adc-cc139255687c","related_kind":"CONTACT","status":"OPEN","title":"Send pricing proposal","type":"Email","type_id":"099104fc-dcee-57bf-b906-d2b826beb51d"})).unwrap_or_default(), serde_json::from_value::<Task>(serde_json::json!({"completed_at":"2026-11-05T16:00:00Z","contact_id":"b7ad6bb7-2c27-526b-8340-a18d60fb176b","contact_name":"Oliver Brown","created_at":"2026-10-29T16:00:00Z","created_by":"sam","deal_id":"c50637e3-5f8b-5d72-bb89-8fe81b9eb36f","deal_title":"Alpine Ski · migration","due_at":"2026-11-06T16:00:00Z","id":"a747276f-e470-5c47-9a27-1129890e7282","outcome":"Outcome","overdue":false,"owner_name":"Sam Lee","related_id":"57d59745-33ec-5dc8-9adc-cc139255687c","related_kind":"CONTACT","status":"OPEN","title":"Demo","type":"Meeting","type_id":"645bfe17-69c3-558e-82cd-9594c3609fa2"})).unwrap_or_default()],limit: 25,page: 1,total: 8,
    })
}
