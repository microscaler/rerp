// User-owned controller for handler 'create_collection_case'.

use crate::handlers::create_collection_case::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCollectionCaseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        assigned_to: "example".to_string(),
        customer_id: "example".to_string(),
        id: "example".to_string(),
        status: "example".to_string(),
        total_due: 3.14,
    })
}
