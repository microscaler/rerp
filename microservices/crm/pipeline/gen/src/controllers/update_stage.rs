// User-owned controller for handler 'update_stage'.

use crate::handlers::update_stage::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateStageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        color: Some(42),
        description: Some("example".to_string()),
        fold: Some(true),
        id: "example".to_string(),
        is_lost: Some(true),
        is_won: Some(true),
        lead_count: Some(42),
        name: "example".to_string(),
        probability: 42,
        requirements: Some("example".to_string()),
        rotting_threshold_days: Some(42),
        sequence: 42,
        team_ids: Some(vec![]),
    })
}
