// User-owned controller for handler 'list_templates'.

use crate::handlers::list_templates::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Template;

#[handler(ListTemplatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "body": "Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
    //       "body_format": "MARKDOWN",
    //       "channel": "EMAIL",
    //       "created_at": "2026-07-08T16:00:00Z",
    //       "id": "736685cb-324b-572c-b287-deb656a4d8de",
    //       "is_archived": false,
    //       "kind": "AUTOMATION",
    //       "merge_fields": [
    //         "contact.first_name",
    //         "sender.postal_address",
    //         "links.unsubscribe"
    //       ],
    //       "name": "Welcome · day 0: what you can do today",
    //       "preheader": "",
    //       "subject": "Welcome to {{sender.name}}",
    //       "updated_at": "2026-10-16T16:00:00Z",
    //       "version": 3
    //     },
    //     {
    //       "body": "Hi {{contact.first_name}},\n\nWelcome · day 2: three things to set up.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
    //       "body_format": "MARKDOWN",
    //       "channel": "EMAIL",
    //       "created_at": "2026-07-13T16:00:00Z",
    //       "id": "f248dd12-4837-5fde-aa51-2fba5a0e8227",
    //       "is_archived": false,
    //       "kind": "AUTOMATION",
    //       "merge_fields": [
    //         "contact.first_name",
    //         "sender.postal_address",
    //         "links.unsubscribe"
    //       ],
    //       "name": "Welcome · day 2: three things to set up",
    //       "preheader": "",
    //       "subject": "Three things worth setting up",
    //       "updated_at": "2026-10-17T16:00:00Z",
    //       "version": 2
    //     },
    //     {
    //       "body": "Hi {{contact.first_name}},\n\nDay 2 · resend with new subject.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
    //       "body_format": "MARKDOWN",
    //       "channel": "EMAIL",
    //       "created_at": "2026-07-18T16:00:00Z",
    //       "id": "40072573-6c82-5100-8709-5ed36d8c6794",
    //       "is_archived": false,
    //       "kind": "AUTOMATION",
    //       "merge_fields": [
    //         "contact.first_name",
    //         "sender.postal_address",
    //         "links.unsubscribe"
    //       ],
    //       "name": "Day 2 · resend with new subject",
    //       "preheader": "",
    //       "subject": "Still here? Three quick wins",
    //       "updated_at": "2026-10-18T16:00:00Z",
    //       "version": 1
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 16
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
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
    },
    {
      "body": "Hi {{contact.first_name}},\n\nWelcome · day 2: three things to set up.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
      "body_format": "MARKDOWN",
      "channel": "EMAIL",
      "created_at": "2026-07-13T16:00:00Z",
      "id": "f248dd12-4837-5fde-aa51-2fba5a0e8227",
      "is_archived": false,
      "kind": "AUTOMATION",
      "merge_fields": [
        "contact.first_name",
        "sender.postal_address",
        "links.unsubscribe"
      ],
      "name": "Welcome · day 2: three things to set up",
      "preheader": "",
      "subject": "Three things worth setting up",
      "updated_at": "2026-10-17T16:00:00Z",
      "version": 2
    },
    {
      "body": "Hi {{contact.first_name}},\n\nDay 2 · resend with new subject.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})",
      "body_format": "MARKDOWN",
      "channel": "EMAIL",
      "created_at": "2026-07-18T16:00:00Z",
      "id": "40072573-6c82-5100-8709-5ed36d8c6794",
      "is_archived": false,
      "kind": "AUTOMATION",
      "merge_fields": [
        "contact.first_name",
        "sender.postal_address",
        "links.unsubscribe"
      ],
      "name": "Day 2 · resend with new subject",
      "preheader": "",
      "subject": "Still here? Three quick wins",
      "updated_at": "2026-10-18T16:00:00Z",
      "version": 1
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 16
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","created_at":"2026-07-08T16:00:00Z","id":"736685cb-324b-572c-b287-deb656a4d8de","is_archived":false,"kind":"AUTOMATION","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Welcome · day 0: what you can do today","preheader":"","subject":"Welcome to {{sender.name}}","updated_at":"2026-10-16T16:00:00Z","version":3})).unwrap_or_default(), serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nWelcome · day 2: three things to set up.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","created_at":"2026-07-13T16:00:00Z","id":"f248dd12-4837-5fde-aa51-2fba5a0e8227","is_archived":false,"kind":"AUTOMATION","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Welcome · day 2: three things to set up","preheader":"","subject":"Three things worth setting up","updated_at":"2026-10-17T16:00:00Z","version":2})).unwrap_or_default(), serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nDay 2 · resend with new subject.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","created_at":"2026-07-18T16:00:00Z","id":"40072573-6c82-5100-8709-5ed36d8c6794","is_archived":false,"kind":"AUTOMATION","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Day 2 · resend with new subject","preheader":"","subject":"Still here? Three quick wins","updated_at":"2026-10-18T16:00:00Z","version":1})).unwrap_or_default()],limit: 25,page: 1,total: 16,
    })
}
