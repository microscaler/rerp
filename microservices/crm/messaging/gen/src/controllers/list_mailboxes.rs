// User-owned controller for handler 'list_mailboxes'.

use crate::handlers::list_mailboxes::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Mailbox;

#[handler(ListMailboxesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "address": "alex@acme.example",
    //       "created_at": "2026-10-01T16:00:00Z",
    //       "display_name": "Alex Morgan",
    //       "id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //       "last_error": "Last error",
    //       "last_sync_at": "2026-11-05T15:58:00Z",
    //       "log_to_deals": true,
    //       "provider": "GOOGLE",
    //       "shared_with_user_ids": [],
    //       "signature_html": "Alex Morgan · Acme Co · +359 2 000 0000",
    //       "status": "CONNECTED",
    //       "sync_enabled": true,
    //       "sync_scope": "KNOWN_CONTACTS",
    //       "sync_since": "2026-10-01T00:00:00Z",
    //       "track_clicks": true,
    //       "track_opens": true,
    //       "user_id": "alex"
    //     },
    //     {
    //       "address": "sam@acme.example",
    //       "created_at": "2026-09-15T16:00:00Z",
    //       "display_name": "Sam Lee",
    //       "id": "c2fb563e-65c5-5c5a-a0cf-b17ae70bb727",
    //       "last_error": "Last error",
    //       "last_sync_at": "2026-11-05T15:54:00Z",
    //       "log_to_deals": false,
    //       "provider": "MICROSOFT",
    //       "shared_with_user_ids": [],
    //       "signature_html": "",
    //       "status": "CONNECTED",
    //       "sync_enabled": true,
    //       "sync_scope": "KNOWN_CONTACTS",
    //       "sync_since": "2026-09-15T00:00:00Z",
    //       "track_clicks": false,
    //       "track_opens": true,
    //       "user_id": "sam"
    //     },
    //     {
    //       "address": "sales@acme.example",
    //       "created_at": "2026-08-01T16:00:00Z",
    //       "display_name": "Acme Sales",
    //       "id": "b9d592b7-1ff3-5f0c-ae30-4f183030d442",
    //       "imap": {
    //         "imap_host": "imap.acme.example",
    //         "imap_port": 993,
    //         "smtp_host": "smtp.acme.example",
    //         "smtp_port": 587,
    //         "use_tls": true,
    //         "username": "sales@acme.example"
    //       },
    //       "last_error": "AUTHENTICATIONFAILED: invalid credentials",
    //       "last_sync_at": "2026-11-02T16:00:00Z",
    //       "log_to_deals": true,
    //       "provider": "IMAP",
    //       "shared_with_user_ids": [
    //         "sam",
    //         "riya"
    //       ],
    //       "signature_html": "",
    //       "status": "ERROR",
    //       "sync_enabled": true,
    //       "sync_scope": "ALL",
    //       "sync_since": "2026-08-01T00:00:00Z",
    //       "track_clicks": true,
    //       "track_opens": true,
    //       "user_id": "alex"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
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
    },
    {
      "address": "sam@acme.example",
      "created_at": "2026-09-15T16:00:00Z",
      "display_name": "Sam Lee",
      "id": "c2fb563e-65c5-5c5a-a0cf-b17ae70bb727",
      "last_error": "Last error",
      "last_sync_at": "2026-11-05T15:54:00Z",
      "log_to_deals": false,
      "provider": "MICROSOFT",
      "shared_with_user_ids": [],
      "signature_html": "",
      "status": "CONNECTED",
      "sync_enabled": true,
      "sync_scope": "KNOWN_CONTACTS",
      "sync_since": "2026-09-15T00:00:00Z",
      "track_clicks": false,
      "track_opens": true,
      "user_id": "sam"
    },
    {
      "address": "sales@acme.example",
      "created_at": "2026-08-01T16:00:00Z",
      "display_name": "Acme Sales",
      "id": "b9d592b7-1ff3-5f0c-ae30-4f183030d442",
      "imap": {
        "imap_host": "imap.acme.example",
        "imap_port": 993,
        "smtp_host": "smtp.acme.example",
        "smtp_port": 587,
        "use_tls": true,
        "username": "sales@acme.example"
      },
      "last_error": "AUTHENTICATIONFAILED: invalid credentials",
      "last_sync_at": "2026-11-02T16:00:00Z",
      "log_to_deals": true,
      "provider": "IMAP",
      "shared_with_user_ids": [
        "sam",
        "riya"
      ],
      "signature_html": "",
      "status": "ERROR",
      "sync_enabled": true,
      "sync_scope": "ALL",
      "sync_since": "2026-08-01T00:00:00Z",
      "track_clicks": true,
      "track_opens": true,
      "user_id": "alex"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Mailbox>(serde_json::json!({"address":"alex@acme.example","created_at":"2026-10-01T16:00:00Z","display_name":"Alex Morgan","id":"22788e76-5b04-5e9c-97a4-49cade135ff3","last_error":"Last error","last_sync_at":"2026-11-05T15:58:00Z","log_to_deals":true,"provider":"GOOGLE","shared_with_user_ids":[],"signature_html":"Alex Morgan · Acme Co · +359 2 000 0000","status":"CONNECTED","sync_enabled":true,"sync_scope":"KNOWN_CONTACTS","sync_since":"2026-10-01T00:00:00Z","track_clicks":true,"track_opens":true,"user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<Mailbox>(serde_json::json!({"address":"sam@acme.example","created_at":"2026-09-15T16:00:00Z","display_name":"Sam Lee","id":"c2fb563e-65c5-5c5a-a0cf-b17ae70bb727","last_error":"Last error","last_sync_at":"2026-11-05T15:54:00Z","log_to_deals":false,"provider":"MICROSOFT","shared_with_user_ids":[],"signature_html":"","status":"CONNECTED","sync_enabled":true,"sync_scope":"KNOWN_CONTACTS","sync_since":"2026-09-15T00:00:00Z","track_clicks":false,"track_opens":true,"user_id":"sam"})).unwrap_or_default(), serde_json::from_value::<Mailbox>(serde_json::json!({"address":"sales@acme.example","created_at":"2026-08-01T16:00:00Z","display_name":"Acme Sales","id":"b9d592b7-1ff3-5f0c-ae30-4f183030d442","imap":{"imap_host":"imap.acme.example","imap_port":993,"smtp_host":"smtp.acme.example","smtp_port":587,"use_tls":true,"username":"sales@acme.example"},"last_error":"AUTHENTICATIONFAILED: invalid credentials","last_sync_at":"2026-11-02T16:00:00Z","log_to_deals":true,"provider":"IMAP","shared_with_user_ids":["sam","riya"],"signature_html":"","status":"ERROR","sync_enabled":true,"sync_scope":"ALL","sync_since":"2026-08-01T00:00:00Z","track_clicks":true,"track_opens":true,"user_id":"alex"})).unwrap_or_default()],limit: 25,page: 1,total: 4,
    })
}
