// User-owned controller for handler 'get_ad_ltv'.

use crate::handlers::get_ad_ltv::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdLtvController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by: Some("example".to_string()),
        cohorts: Some(vec![]),
    })
}
