// User-owned controller for handler 'create_bulk_job'.

use crate::handlers::create_bulk_job::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BulkJobAction;

#[handler(CreateBulkJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        action: Default::default(),
        by: Some("example".to_string()),
        contacts: Some(42),
        created_at: "example".to_string(),
        field_key: Some("example".to_string()),
        finished_at: Some("example".to_string()),
        id: "example".to_string(),
        label: Some("example".to_string()),
        list_id: Some("example".to_string()),
        segment_id: Some("example".to_string()),
        status: "example".to_string(),
        undo_until: Some("example".to_string()),
        undoable: Some(true),
        value: Some("example".to_string()),
    })
}
