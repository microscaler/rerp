// User-owned controller for handler 'list_follow_up_levels'.

use crate::handlers::list_follow_up_levels::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FollowUpLevel;

#[handler(ListFollowUpLevelsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
