// User-owned controller for handler 'create_collection_activity'.

use crate::handlers::create_collection_activity::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCollectionActivityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        activity_type: "example".to_string(),
        assigned_to: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        customer_invoice_id: "example".to_string(),
        date: "example".to_string(),
        id: "example".to_string(),
        next_follow_up_date: "example".to_string(),
        notes: "example".to_string(),
        updated_at: "example".to_string(),
    })
}
