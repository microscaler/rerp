// User-owned controller for handler 'merge_contacts'.

use crate::handlers::merge_contacts::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(MergeContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "kept_id": "c1caa8c6-e942-5db0-95ba-fc7a108a2c83",
    //   "merged_account_count": 3,
    //   "merged_lead_count": 3,
    //   "removed_id": "61135c5b-7849-5d56-a7ed-e0c866b2aa30"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "kept_id": "c1caa8c6-e942-5db0-95ba-fc7a108a2c83",
  "merged_account_count": 3,
  "merged_lead_count": 3,
  "removed_id": "61135c5b-7849-5d56-a7ed-e0c866b2aa30"
}"#,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        kept_id: "c1caa8c6-e942-5db0-95ba-fc7a108a2c83".to_string(),
        merged_account_count: 3,
        merged_lead_count: 3,
        removed_id: "61135c5b-7849-5d56-a7ed-e0c866b2aa30".to_string(),
    })
}
