// User-owned controller for handler 'get_mailbox'.

use crate::handlers::get_mailbox::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ImapConfig;
#[allow(unused_imports)]
use crate::handlers::types::MailboxProvider;
#[allow(unused_imports)]
use crate::handlers::types::MailboxStatus;

#[handler(GetMailboxController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        address: "example".to_string(),
        created_at: Some("example".to_string()),
        display_name: Some("example".to_string()),
        id: "example".to_string(),
        imap: Some(Default::default()),
        last_error: Some("example".to_string()),
        last_sync_at: Some("example".to_string()),
        log_to_deals: Some(true),
        provider: Default::default(),
        shared_with_user_ids: Some(vec![]),
        signature_html: Some("example".to_string()),
        status: Default::default(),
        sync_enabled: true,
        sync_scope: Some("example".to_string()),
        sync_since: Some("example".to_string()),
        track_clicks: Some(true),
        track_opens: Some(true),
        updated_at: Some("example".to_string()),
        user_id: "example".to_string(),
    })
}
