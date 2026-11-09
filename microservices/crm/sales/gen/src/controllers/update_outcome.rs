// User-owned controller for handler 'update_outcome'.

use crate::handlers::update_outcome::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Sentiment;

#[handler(UpdateOutcomeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        count_30d: Some(42),
        id: "example".to_string(),
        label: "example".to_string(),
        sentiment: Default::default(),
        used_by_type_ids: Some(vec![]),
    })
}
