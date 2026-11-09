// User-owned controller for handler 'create_score'.

use crate::handlers::create_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ScoreRule;

#[handler(CreateScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        counts: Some(Default::default()),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        distribution: Some(vec![]),
        entity: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        rules: Some(vec![]),
        status: "example".to_string(),
        thresholds: Default::default(),
        updated_at: Some("example".to_string()),
    })
}
