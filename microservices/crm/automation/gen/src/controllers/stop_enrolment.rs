// User-owned controller for handler 'stop_enrolment'.

use crate::handlers::stop_enrolment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EnrolmentStatus;
#[allow(unused_imports)]
use crate::handlers::types::EnrolmentStepRecord;
#[allow(unused_imports)]
use crate::handlers::types::ExitReason;
#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(StopEnrolmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        contact_email: Some("example".to_string()),
        contact_id: "example".to_string(),
        current_step_id: Some("example".to_string()),
        ended_at: Some("example".to_string()),
        entered_at: "example".to_string(),
        entered_by: Some("example".to_string()),
        exit_reason: Some(Default::default()),
        goal_step_id: Some("example".to_string()),
        history: Some(vec![]),
        id: "example".to_string(),
        next_run_at: Some("example".to_string()),
        split_paths: Some(Default::default()),
        status: Default::default(),
        trigger_id: Some("example".to_string()),
        trigger_subject: Some("example".to_string()),
        trigger_type: Some(Default::default()),
        workflow_id: "example".to_string(),
        workflow_name: Some("example".to_string()),
    })
}
