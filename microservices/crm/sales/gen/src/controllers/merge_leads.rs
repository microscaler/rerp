// User-owned controller for handler 'merge_leads'.

use crate::handlers::merge_leads::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(MergeLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "kept_id": "d7aeb226-cdfa-5dd7-95f9-e2ada5718092",
    //   "merged_description_additions": 3,
    //   "merged_id": "ee875d84-979f-54f7-85a1-e7ffad9cc6c8",
    //   "merged_tag_count": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "kept_id": "d7aeb226-cdfa-5dd7-95f9-e2ada5718092",
  "merged_description_additions": 3,
  "merged_id": "ee875d84-979f-54f7-85a1-e7ffad9cc6c8",
  "merged_tag_count": 3
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        kept_id: "d7aeb226-cdfa-5dd7-95f9-e2ada5718092".to_string(),
        merged_description_additions: 3,
        merged_id: "ee875d84-979f-54f7-85a1-e7ffad9cc6c8".to_string(),
        merged_tag_count: 3,
    })
}
