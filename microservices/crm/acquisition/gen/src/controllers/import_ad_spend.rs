// User-owned controller for handler 'import_ad_spend'.

use crate::handlers::import_ad_spend::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ImportAdSpendController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: Some("example".to_string()),
        error: Some("example".to_string()),
        finished_at: Some("example".to_string()),
        from: Some("example".to_string()),
        id: Some("example".to_string()),
        rows: Some(42),
        status: Some("example".to_string()),
        to: Some("example".to_string()),
    })
}
