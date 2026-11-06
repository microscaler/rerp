// User-owned controller for handler 'bulk_update_deals'.

use crate::handlers::bulk_update_deals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BulkUpdateDealsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        skipped: Some(vec![]),
        updated: Some(42),
    })
}
