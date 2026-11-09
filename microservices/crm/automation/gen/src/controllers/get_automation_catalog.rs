// User-owned controller for handler 'get_automation_catalog'.

use crate::handlers::get_automation_catalog::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CatalogEntry;

#[handler(GetAutomationCatalogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "actions": [
    //     {
    //       "group": "Sending",
    //       "kind": "ACTION",
    //       "label": "Send email",
    //       "type": "SEND_MESSAGE"
    //     },
    //     {
    //       "group": "Sending",
    //       "kind": "ACTION",
    //       "label": "Send SMS",
    //       "type": "SEND_SMS"
    //     },
    //     {
    //       "group": "Sending",
    //       "kind": "ACTION",
    //       "label": "Send 1:1 email",
    //       "type": "SEND_1TO1_EMAIL"
    //     }
    //   ],
    //   "triggers": [
    //     {
    //       "group": "Behaviors",
    //       "label": "On a schedule",
    //       "type": "TIME_BASED"
    //     },
    //     {
    //       "group": "Email & messages",
    //       "label": "Subscribes to a list",
    //       "type": "SUBSCRIBED_TO_LIST"
    //     },
    //     {
    //       "group": "Email & messages",
    //       "label": "Unsubscribes from a list",
    //       "type": "UNSUBSCRIBED_FROM_LIST"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "actions": [
    {
      "group": "Sending",
      "kind": "ACTION",
      "label": "Send email",
      "type": "SEND_MESSAGE"
    },
    {
      "group": "Sending",
      "kind": "ACTION",
      "label": "Send SMS",
      "type": "SEND_SMS"
    },
    {
      "group": "Sending",
      "kind": "ACTION",
      "label": "Send 1:1 email",
      "type": "SEND_1TO1_EMAIL"
    }
  ],
  "triggers": [
    {
      "group": "Behaviors",
      "label": "On a schedule",
      "type": "TIME_BASED"
    },
    {
      "group": "Email & messages",
      "label": "Subscribes to a list",
      "type": "SUBSCRIBED_TO_LIST"
    },
    {
      "group": "Email & messages",
      "label": "Unsubscribes from a list",
      "type": "UNSUBSCRIBED_FROM_LIST"
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
        actions: vec![serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send email","type":"SEND_MESSAGE"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send SMS","type":"SEND_SMS"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send 1:1 email","type":"SEND_1TO1_EMAIL"})).unwrap_or_default()],triggers: vec![serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Behaviors","label":"On a schedule","type":"TIME_BASED"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Email & messages","label":"Subscribes to a list","type":"SUBSCRIBED_TO_LIST"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Email & messages","label":"Unsubscribes from a list","type":"UNSUBSCRIBED_FROM_LIST"})).unwrap_or_default()],
    })
}
