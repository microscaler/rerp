// User-owned controller for handler 'merge_leads'.

use crate::handlers::merge_leads::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(MergeLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        kept_id: "example".to_string(),
        merged_description_additions: 42,
        merged_id: "example".to_string(),
        merged_tag_count: 42,
    })
}
