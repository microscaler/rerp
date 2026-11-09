// User-owned controller for handler 'import_contacts'.

use crate::handlers::import_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ImportContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created: Some(42),
        created_at: "example".to_string(),
        download_url: Some("example".to_string()),
        errors: Some(vec![]),
        finished_at: Some("example".to_string()),
        id: "example".to_string(),
        kind: "example".to_string(),
        processed_rows: Some(42),
        skipped: Some(42),
        status: "example".to_string(),
        total_rows: Some(42),
        updated: Some(42),
    })
}
