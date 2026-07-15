// User-owned controller for handler 'bulk_depreciate'.

use crate::handlers::bulk_depreciate::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BulkDepreciateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        assets_processed: Some(42),
        errors: Some(vec![]),
        total_depreciation: Some(3.14),
        total_entries: Some(42),
    })
}
