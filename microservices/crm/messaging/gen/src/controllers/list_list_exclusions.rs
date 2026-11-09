// User-owned controller for handler 'list_list_exclusions'.

use crate::handlers::list_list_exclusions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ListExclusion;

#[handler(ListListExclusionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-09-04T16:00:00Z",
    //       "created_by": "Alex Morgan",
    //       "id": "50137312-86a6-50be-b4f3-784df3c7ee21",
    //       "list_ids": [],
    //       "list_names": [
    //         "All lists"
    //       ],
    //       "pattern": "*@competitor.example",
    //       "reason": "Competitor domain"
    //     },
    //     {
    //       "created_at": "2026-03-12T16:00:00Z",
    //       "created_by": "Alex Morgan",
    //       "id": "2e167db1-8e7c-535c-aea9-634a21d3d38f",
    //       "list_ids": [],
    //       "list_names": [
    //         "Newsletter",
    //         "Webinars"
    //       ],
    //       "pattern": "*@acme.example",
    //       "reason": "Staff should not receive marketing"
    //     },
    //     {
    //       "created_at": "2026-10-12T16:00:00Z",
    //       "created_by": "Sam Lee",
    //       "id": "2c4bae6a-cdaf-5a49-beb4-5e3c1858a501",
    //       "list_ids": [],
    //       "list_names": [
    //         "Webinars"
    //       ],
    //       "pattern": "ceo@contoso.example",
    //       "reason": "Account manager request"
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
      "created_at": "2026-09-04T16:00:00Z",
      "created_by": "Alex Morgan",
      "id": "50137312-86a6-50be-b4f3-784df3c7ee21",
      "list_ids": [],
      "list_names": [
        "All lists"
      ],
      "pattern": "*@competitor.example",
      "reason": "Competitor domain"
    },
    {
      "created_at": "2026-03-12T16:00:00Z",
      "created_by": "Alex Morgan",
      "id": "2e167db1-8e7c-535c-aea9-634a21d3d38f",
      "list_ids": [],
      "list_names": [
        "Newsletter",
        "Webinars"
      ],
      "pattern": "*@acme.example",
      "reason": "Staff should not receive marketing"
    },
    {
      "created_at": "2026-10-12T16:00:00Z",
      "created_by": "Sam Lee",
      "id": "2c4bae6a-cdaf-5a49-beb4-5e3c1858a501",
      "list_ids": [],
      "list_names": [
        "Webinars"
      ],
      "pattern": "ceo@contoso.example",
      "reason": "Account manager request"
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
        items: vec![serde_json::from_value::<ListExclusion>(serde_json::json!({"created_at":"2026-09-04T16:00:00Z","created_by":"Alex Morgan","id":"50137312-86a6-50be-b4f3-784df3c7ee21","list_ids":[],"list_names":["All lists"],"pattern":"*@competitor.example","reason":"Competitor domain"})).unwrap_or_default(), serde_json::from_value::<ListExclusion>(serde_json::json!({"created_at":"2026-03-12T16:00:00Z","created_by":"Alex Morgan","id":"2e167db1-8e7c-535c-aea9-634a21d3d38f","list_ids":[],"list_names":["Newsletter","Webinars"],"pattern":"*@acme.example","reason":"Staff should not receive marketing"})).unwrap_or_default(), serde_json::from_value::<ListExclusion>(serde_json::json!({"created_at":"2026-10-12T16:00:00Z","created_by":"Sam Lee","id":"2c4bae6a-cdaf-5a49-beb4-5e3c1858a501","list_ids":[],"list_names":["Webinars"],"pattern":"ceo@contoso.example","reason":"Account manager request"})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
