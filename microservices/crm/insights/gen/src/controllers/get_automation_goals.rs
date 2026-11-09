// User-owned controller for handler 'get_automation_goals'.

use crate::handlers::get_automation_goals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationGoalRow;

#[handler(GetAutomationGoalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        goals: vec![],
        workflow_id: "example".to_string(),
    })
}
