// User-owned controller for handler 'create_template'.

use crate::handlers::create_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BodyFormat;
#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::TemplateKind;

#[handler(CreateTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
    //   "body_format": "MARKDOWN",
    //   "channel": "EMAIL",
    //   "click_rate": 0.14,
    //   "created_at": "2026-07-08T16:00:00Z",
    //   "id": "736685cb-324b-572c-b287-deb656a4d8de",
    //   "is_archived": false,
    //   "kind": "AUTOMATION",
    //   "language_code": "en",
    //   "merge_fields": [
    //     "contact.first_name",
    //     "sender.postal_address",
    //     "links.unsubscribe"
    //   ],
    //   "name": "Welcome · day 0: what you can do today",
    //   "open_rate": 0.62,
    //   "preheader": "",
    //   "priority": "NORMAL",
    //   "sent_30d": 1640,
    //   "subject": "Welcome to {{sender.name}}",
    //   "unsub_30d": 2,
    //   "updated_at": "2026-10-16T16:00:00Z",
    //   "used_by": [
    //     {
    //       "step": 1,
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     }
    //   ],
    //   "version": 3,
    //   "versions": [
    //     {
    //       "at": "2026-10-19T16:00:00Z",
    //       "version": 3
    //     },
    //     {
    //       "at": "2026-09-09T16:00:00Z",
    //       "version": 2
    //     },
    //     {
    //       "at": "2026-07-31T16:00:00Z",
    //       "version": 1
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body": "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
  "body_format": "MARKDOWN",
  "channel": "EMAIL",
  "click_rate": 0.14,
  "created_at": "2026-07-08T16:00:00Z",
  "id": "736685cb-324b-572c-b287-deb656a4d8de",
  "is_archived": false,
  "kind": "AUTOMATION",
  "language_code": "en",
  "merge_fields": [
    "contact.first_name",
    "sender.postal_address",
    "links.unsubscribe"
  ],
  "name": "Welcome · day 0: what you can do today",
  "open_rate": 0.62,
  "preheader": "",
  "priority": "NORMAL",
  "sent_30d": 1640,
  "subject": "Welcome to {{sender.name}}",
  "unsub_30d": 2,
  "updated_at": "2026-10-16T16:00:00Z",
  "used_by": [
    {
      "step": 1,
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    }
  ],
  "version": 3,
  "versions": [
    {
      "at": "2026-10-19T16:00:00Z",
      "version": 3
    },
    {
      "at": "2026-09-09T16:00:00Z",
      "version": 2
    },
    {
      "at": "2026-07-31T16:00:00Z",
      "version": 1
    }
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
        body: "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})".to_string(),body_format: serde_json::from_value::<BodyFormat>(serde_json::Value::String("MARKDOWN".to_string())).unwrap_or_default(),called_by: Some("example".to_string()),channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string())).unwrap_or_default(),click_rate: Some(0.14),created_at: "2026-07-08T16:00:00Z".to_string(),delivered_rate: Some(1.5),design: Some(Default::default()),id: "736685cb-324b-572c-b287-deb656a4d8de".to_string(),is_archived: Some(false),kind: serde_json::from_value::<TemplateKind>(serde_json::Value::String("AUTOMATION".to_string())).unwrap_or_default(),language_code: Some("en".to_string()),merge_fields: Some(vec!["contact.first_name".to_string(), "sender.postal_address".to_string(), "links.unsubscribe".to_string()]),name: "Welcome · day 0: what you can do today".to_string(),open_rate: Some(0.62),preheader: Some("".to_string()),priority: Some("NORMAL".to_string()),sent_30d: Some(1640),subject: Some("Welcome to {{sender.name}}".to_string()),tag: Some("example".to_string()),text_body: Some("example".to_string()),thumbnail_url: Some("example".to_string()),unsub_30d: Some(2),updated_at: Some("2026-10-16T16:00:00Z".to_string()),used_by: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"step":1,"workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"}"#).unwrap_or(serde_json::Value::Null)]),version: 3,versions: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-10-19T16:00:00Z","version":3}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-09-09T16:00:00Z","version":2}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-07-31T16:00:00Z","version":1}"#).unwrap_or(serde_json::Value::Null)]),
    })
}
