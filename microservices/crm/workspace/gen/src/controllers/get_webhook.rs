// User-owned controller for handler 'get_webhook'.

use crate::handlers::get_webhook::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WebhookDeliveryStatus;

#[handler(GetWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-08-07T16:00:00Z",
    //   "events": [
    //     "contact.subscribed",
    //     "contact.unsubscribed"
    //   ],
    //   "failures": 0,
    //   "id": "9a087430-47c9-5317-8abb-3f6fe7a9cbf9",
    //   "is_active": true,
    //   "name": "Northwind Traders",
    //   "status": "OK",
    //   "url": "https://api.acme.example/hooks/crm"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-08-07T16:00:00Z",
  "events": [
    "contact.subscribed",
    "contact.unsubscribed"
  ],
  "failures": 0,
  "id": "9a087430-47c9-5317-8abb-3f6fe7a9cbf9",
  "is_active": true,
  "name": "Northwind Traders",
  "status": "OK",
  "url": "https://api.acme.example/hooks/crm"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("2026-08-07T16:00:00Z".to_string()),
        detail: Some("example".to_string()),
        events: vec![
            "contact.subscribed".to_string(),
            "contact.unsubscribed".to_string(),
        ],
        failures: Some(0),
        headers: Some(Default::default()),
        id: "9a087430-47c9-5317-8abb-3f6fe7a9cbf9".to_string(),
        is_active: true,
        last_status: Some(Default::default()),
        last_triggered: Some("example".to_string()),
        name: "Northwind Traders".to_string(),
        retry_count: Some(42),
        secret: Some("example".to_string()),
        status: Some("OK".to_string()),
        url: "https://api.acme.example/hooks/crm".to_string(),
    })
}
