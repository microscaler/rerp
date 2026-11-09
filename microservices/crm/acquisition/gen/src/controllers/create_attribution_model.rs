// User-owned controller for handler 'create_attribution_model'.

use crate::handlers::create_attribution_model::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAttributionModelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        builtin: Some(true),
        count_kinds: Some(vec![]),
        half_life_days: Some(1.5),
        id: Some("example".to_string()),
        is_default: Some(true),
        kind: Some("example".to_string()),
        name: "example".to_string(),
        weights: Some(Default::default()),
        window_days: Some(42),
    })
}
