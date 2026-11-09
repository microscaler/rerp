// User-owned controller for handler 'send_transactional'.

use crate::handlers::send_transactional::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::MessageEvent;
#[allow(unused_imports)]
use crate::handlers::types::Provider;
#[allow(unused_imports)]
use crate::handlers::types::SendStatus;

#[handler(SendTransactionalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        campaign_id: Some("example".to_string()),
        channel: Default::default(),
        clicked_at: Some("example".to_string()),
        contact_id: "example".to_string(),
        created_at: "example".to_string(),
        enrolment_id: Some("example".to_string()),
        error: Some("example".to_string()),
        events: Some(vec![]),
        id: "example".to_string(),
        opened_at: Some("example".to_string()),
        provider: Some(Default::default()),
        provider_message_id: Some("example".to_string()),
        status: Default::default(),
        subject: Some("example".to_string()),
        template_id: Some("example".to_string()),
        template_version: Some(42),
        to: Some("example".to_string()),
        variant_key: Some("example".to_string()),
        workflow_id: Some("example".to_string()),
    })
}
