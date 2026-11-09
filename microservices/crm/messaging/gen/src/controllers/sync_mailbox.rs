// User-owned controller for handler 'sync_mailbox'.

use crate::handlers::sync_mailbox::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ImapConfig;
#[allow(unused_imports)]
use crate::handlers::types::MailboxProvider;
#[allow(unused_imports)]
use crate::handlers::types::MailboxStatus;

#[handler(SyncMailboxController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "address": "alex@acme.example",
    //   "created_at": "2026-10-01T16:00:00Z",
    //   "display_name": "Alex Morgan",
    //   "id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //   "last_error": "Last error",
    //   "last_sync_at": "2026-11-05T15:58:00Z",
    //   "log_to_deals": true,
    //   "provider": "GOOGLE",
    //   "shared_with_user_ids": [],
    //   "signature_html": "Alex Morgan · Acme Co · +359 2 000 0000",
    //   "status": "CONNECTED",
    //   "sync_enabled": true,
    //   "sync_scope": "KNOWN_CONTACTS",
    //   "sync_since": "2026-10-01T00:00:00Z",
    //   "track_clicks": true,
    //   "track_opens": true,
    //   "user_id": "alex"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "address": "alex@acme.example",
  "created_at": "2026-10-01T16:00:00Z",
  "display_name": "Alex Morgan",
  "id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
  "last_error": "Last error",
  "last_sync_at": "2026-11-05T15:58:00Z",
  "log_to_deals": true,
  "provider": "GOOGLE",
  "shared_with_user_ids": [],
  "signature_html": "Alex Morgan · Acme Co · +359 2 000 0000",
  "status": "CONNECTED",
  "sync_enabled": true,
  "sync_scope": "KNOWN_CONTACTS",
  "sync_since": "2026-10-01T00:00:00Z",
  "track_clicks": true,
  "track_opens": true,
  "user_id": "alex"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        address: "alex@acme.example".to_string(),
        created_at: Some("2026-10-01T16:00:00Z".to_string()),
        display_name: Some("Alex Morgan".to_string()),
        id: "22788e76-5b04-5e9c-97a4-49cade135ff3".to_string(),
        imap: Some(Default::default()),
        last_error: Some("Last error".to_string()),
        last_sync_at: Some("2026-11-05T15:58:00Z".to_string()),
        log_to_deals: Some(true),
        provider: serde_json::from_value::<MailboxProvider>(serde_json::Value::String(
            "GOOGLE".to_string(),
        ))
        .unwrap_or_default(),
        shared_with_user_ids: Some(vec![]),
        signature_html: Some("Alex Morgan · Acme Co · +359 2 000 0000".to_string()),
        status: serde_json::from_value::<MailboxStatus>(serde_json::Value::String(
            "CONNECTED".to_string(),
        ))
        .unwrap_or_default(),
        sync_enabled: true,
        sync_scope: Some("KNOWN_CONTACTS".to_string()),
        sync_since: Some("2026-10-01T00:00:00Z".to_string()),
        track_clicks: Some(true),
        track_opens: Some(true),
        updated_at: Some("example".to_string()),
        user_id: "alex".to_string(),
    })
}
