// User-owned controller for handler 'run_depreciation'.

use crate::handlers::run_depreciation::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RunDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accumulated_depreciation: 3.14,
        asset_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        depreciation_amount: 3.14,
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        net_book_value: Some(3.14),
        period: "example".to_string(),
        posted_to_gl: Some(true),
        schedule_id: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
