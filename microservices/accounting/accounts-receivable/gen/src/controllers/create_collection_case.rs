// User-owned controller for handler 'create_collection_case'.

use crate::handlers::create_collection_case::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCollectionCaseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        assigned_to: Some("example".to_string()),
        customer_id: "example".to_string(),
        id: "example".to_string(),
        status: "example".to_string(),
        total_due: Some(3.14),
    }
}
