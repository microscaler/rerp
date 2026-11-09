// User-owned controller for handler 'get_open_trends'.

use crate::handlers::get_open_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetOpenTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_hour: vec![],
        by_weekday: vec![],
        timezone: Some("example".to_string()),
    })
}
