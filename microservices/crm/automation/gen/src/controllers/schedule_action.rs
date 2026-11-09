// User-owned controller for handler 'schedule_action'.

use crate::handlers::schedule_action::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ExecutionStatus;

#[handler(ScheduleActionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        action_config: Some(Default::default()),
        created_at: Some("example".to_string()),
        executed_at: Some("example".to_string()),
        id: "example".to_string(),
        scheduled_at: "example".to_string(),
        status: Default::default(),
        workflow_id: Some("example".to_string()),
    })
}
