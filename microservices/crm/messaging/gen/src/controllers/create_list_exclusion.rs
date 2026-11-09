// User-owned controller for handler 'create_list_exclusion'.

use crate::handlers::create_list_exclusion::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateListExclusionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-09-04T16:00:00Z",
    //   "created_by": "Alex Morgan",
    //   "id": "50137312-86a6-50be-b4f3-784df3c7ee21",
    //   "list_ids": [],
    //   "list_names": [
    //     "All lists"
    //   ],
    //   "pattern": "*@competitor.example",
    //   "reason": "Competitor domain"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-09-04T16:00:00Z",
  "created_by": "Alex Morgan",
  "id": "50137312-86a6-50be-b4f3-784df3c7ee21",
  "list_ids": [],
  "list_names": [
    "All lists"
  ],
  "pattern": "*@competitor.example",
  "reason": "Competitor domain"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("2026-09-04T16:00:00Z".to_string()),
        created_by: Some("Alex Morgan".to_string()),
        id: "50137312-86a6-50be-b4f3-784df3c7ee21".to_string(),
        list_ids: Some(vec![]),
        list_names: Some(vec!["All lists".to_string()]),
        pattern: "*@competitor.example".to_string(),
        reason: Some("Competitor domain".to_string()),
    })
}
