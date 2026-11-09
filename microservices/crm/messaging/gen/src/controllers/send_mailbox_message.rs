// User-owned controller for handler 'send_mailbox_message'.

use crate::handlers::send_mailbox_message::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MessageAttachment;

#[handler(SendMailboxMessageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        attachments: Some(vec![]),
        body_html: Some("example".to_string()),
        cc: Some(vec![]),
        clicks: Some(42),
        contact_ids: Some(vec![]),
        deal_ids: Some(vec![]),
        direction: "example".to_string(),
        external_id: Some("example".to_string()),
        from: "example".to_string(),
        id: "example".to_string(),
        mailbox_id: "example".to_string(),
        opens: Some(42),
        replied: Some(true),
        sent_at: "example".to_string(),
        snippet: Some("example".to_string()),
        subject: "example".to_string(),
        thread_id: Some("example".to_string()),
        to: vec![],
    })
}
