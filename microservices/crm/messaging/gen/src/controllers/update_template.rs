// User-owned controller for handler 'update_template'.

use crate::handlers::update_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BodyFormat;
#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::TemplateKind;

#[handler(UpdateTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
    //   "body_format": "MARKDOWN",
    //   "channel": "EMAIL",
    //   "created_at": "2026-07-08T16:00:00Z",
    //   "id": "736685cb-324b-572c-b287-deb656a4d8de",
    //   "is_archived": false,
    //   "kind": "AUTOMATION",
    //   "merge_fields": [
    //     "contact.first_name",
    //     "sender.postal_address",
    //     "links.unsubscribe"
    //   ],
    //   "name": "Welcome · day 0: what you can do today",
    //   "preheader": "",
    //   "subject": "Welcome to {{sender.name}}",
    //   "updated_at": "2026-10-16T16:00:00Z",
    //   "version": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body": "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
  "body_format": "MARKDOWN",
  "channel": "EMAIL",
  "created_at": "2026-07-08T16:00:00Z",
  "id": "736685cb-324b-572c-b287-deb656a4d8de",
  "is_archived": false,
  "kind": "AUTOMATION",
  "merge_fields": [
    "contact.first_name",
    "sender.postal_address",
    "links.unsubscribe"
  ],
  "name": "Welcome · day 0: what you can do today",
  "preheader": "",
  "subject": "Welcome to {{sender.name}}",
  "updated_at": "2026-10-16T16:00:00Z",
  "version": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        body: "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})".to_string(),body_format: serde_json::from_value::<BodyFormat>(serde_json::Value::String("MARKDOWN".to_string())).unwrap_or_default(),channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string())).unwrap_or_default(),created_at: "2026-07-08T16:00:00Z".to_string(),design: Some(Default::default()),id: "736685cb-324b-572c-b287-deb656a4d8de".to_string(),is_archived: Some(false),kind: serde_json::from_value::<TemplateKind>(serde_json::Value::String("AUTOMATION".to_string())).unwrap_or_default(),merge_fields: Some(vec!["contact.first_name".to_string(), "sender.postal_address".to_string(), "links.unsubscribe".to_string()]),name: "Welcome · day 0: what you can do today".to_string(),preheader: Some("".to_string()),subject: Some("Welcome to {{sender.name}}".to_string()),text_body: Some("example".to_string()),thumbnail_url: Some("example".to_string()),updated_at: Some("2026-10-16T16:00:00Z".to_string()),version: 3,
    })
}
