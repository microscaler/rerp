// User-owned controller for handler 'bulk_update_tasks'.

use crate::handlers::bulk_update_tasks::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BulkUpdateTasksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        skipped: Some(vec![]),
        updated: Some(42),
    })
}
