// User-owned controller for handler 'create_follow_up_level'.

use crate::handlers::create_follow_up_level::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFollowUpLevelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        action_type: Some("example".to_string()),
        active: Some(true),
        days_overdue: 42,
        id: "example".to_string(),
        name: "example".to_string(),
        sequence: 42,
    }
}
