// User-owned controller for handler 'reopen_task'.

use crate::handlers::reopen_task::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationHook;
#[allow(unused_imports)]
use crate::handlers::types::RelatedKind;
#[allow(unused_imports)]
use crate::handlers::types::Reminder;
#[allow(unused_imports)]
use crate::handlers::types::Sentiment;
#[allow(unused_imports)]
use crate::handlers::types::TaskStatus;

#[handler(ReopenTaskController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        assignee_id: Some("example".to_string()),
        automation: Some(Default::default()),
        completed_at: Some("example".to_string()),
        completed_by: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        contact_name: Some("example".to_string()),
        created_at: "example".to_string(),
        created_by: Some("example".to_string()),
        deal_id: Some("example".to_string()),
        deal_title: Some("example".to_string()),
        description: Some("example".to_string()),
        due_at: Some("example".to_string()),
        id: "example".to_string(),
        outcome: Some("example".to_string()),
        outcome_id: Some("example".to_string()),
        outcome_sentiment: Some(Default::default()),
        overdue: Some(true),
        owner_name: Some("example".to_string()),
        related_id: "example".to_string(),
        related_kind: Default::default(),
        reminders: Some(vec![]),
        status: Default::default(),
        title: "example".to_string(),
        r#type: Some("example".to_string()),
        type_id: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
