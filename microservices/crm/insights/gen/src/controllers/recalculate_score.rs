// User-owned controller for handler 'recalculate_score'.

use crate::handlers::recalculate_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecalculateScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        queued_at: Some("example".to_string()),
        score_id: "example".to_string(),
        status: "example".to_string(),
    })
}
