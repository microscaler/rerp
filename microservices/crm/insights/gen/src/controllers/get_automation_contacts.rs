// User-owned controller for handler 'get_automation_contacts'.

use crate::handlers::get_automation_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CompletionBucket;
#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::PaginatedAutomationContactRow;

#[handler(GetAutomationContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        avg_seconds_to_complete: Some(42),
        completion_rate: 1.5,
        completion_rate_change_30d: Some(1.5),
        completion_times: Some(vec![]),
        contacts: Some(Default::default()),
        contacts_entered: 42,
        top_lists: Some(vec![]),
        top_tags: Some(vec![]),
        workflow_id: "example".to_string(),
    })
}
