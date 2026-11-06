// User-owned controller for handler 'get_stage'.

use crate::handlers::get_stage::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetStageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        card_fields: Some(vec![]),
        color: Some("example".to_string()),
        deal_order: Some("example".to_string()),
        description: Some("example".to_string()),
        fold: Some(true),
        id: "example".to_string(),
        is_lost: Some(true),
        is_won: Some(true),
        lead_count: Some(42),
        name: "example".to_string(),
        pipeline_id: Some("example".to_string()),
        probability: 42,
        requirements: Some("example".to_string()),
        rotting_days: Some(42),
        rotting_threshold_days: Some(42),
        sequence: 42,
        team_ids: Some(vec![]),
    }
}
