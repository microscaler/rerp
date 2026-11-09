// User-owned controller for handler 'merge_contacts'.

use crate::handlers::merge_contacts::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(MergeContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        kept_id: "example".to_string(),
        merged_account_count: 42,
        merged_lead_count: 42,
        removed_id: "example".to_string(),
    })
}
