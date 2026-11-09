// User-owned controller for handler 'explain_score'.

use crate::handlers::explain_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ExplainScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        band: Some("example".to_string()),
        entity_id: "example".to_string(),
        entity_name: Some("example".to_string()),
        factors: vec![],
        score: 42,
        score_id: "example".to_string(),
    })
}
