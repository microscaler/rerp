// User-owned controller for handler 'send_mailbox_message'.

use crate::handlers::send_mailbox_message::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MessageAttachment;

#[handler(SendMailboxMessageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "direction": "INBOUND",
    //   "from": "From",
    //   "id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //   "mailbox_id": "e37ade25-ae07-5176-b130-808e945e6fc5",
    //   "sent_at": "2026-11-05T16:00:00Z",
    //   "subject": "October product update",
    //   "to": [
    //     "To"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "direction": "INBOUND",
  "from": "From",
  "id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
  "mailbox_id": "e37ade25-ae07-5176-b130-808e945e6fc5",
  "sent_at": "2026-11-05T16:00:00Z",
  "subject": "October product update",
  "to": [
    "To"
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        attachments: Some(vec![]),
        body_html: Some("example".to_string()),
        cc: Some(vec![]),
        clicks: Some(42),
        contact_ids: Some(vec![]),
        contact_name: Some("example".to_string()),
        deal_ids: Some(vec![]),
        deal_title: Some("example".to_string()),
        direction: "INBOUND".to_string(),
        external_id: Some("example".to_string()),
        from: "From".to_string(),
        id: "22788e76-5b04-5e9c-97a4-49cade135ff3".to_string(),
        mailbox_id: "e37ade25-ae07-5176-b130-808e945e6fc5".to_string(),
        opens: Some(42),
        replied: Some(true),
        sent_at: "2026-11-05T16:00:00Z".to_string(),
        snippet: Some("example".to_string()),
        subject: "October product update".to_string(),
        thread_id: Some("example".to_string()),
        to: vec!["To".to_string()],
    })
}
