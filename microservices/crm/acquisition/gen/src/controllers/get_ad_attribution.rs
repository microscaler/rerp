// User-owned controller for handler 'get_ad_attribution'.

use crate::handlers::get_ad_attribution::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdAttributionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        from: Some("example".to_string()),
        model: Some("example".to_string()),
        rows: Some(vec![]),
        to: Some("example".to_string()),
        totals: Some(Default::default()),
        warm_up_lift: Some(Default::default()),
        window_days: Some(42),
    })
}
