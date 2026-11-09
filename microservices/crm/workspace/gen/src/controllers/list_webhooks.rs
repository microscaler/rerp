// User-owned controller for handler 'list_webhooks'.

use crate::handlers::list_webhooks::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Webhook;

#[handler(ListWebhooksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-08-07T16:00:00Z",
    //       "events": [
    //         "contact.subscribed",
    //         "contact.unsubscribed"
    //       ],
    //       "id": "9a087430-47c9-5317-8abb-3f6fe7a9cbf9",
    //       "is_active": true,
    //       "name": "Northwind Traders",
    //       "url": "https://api.acme.example/hooks/crm"
    //     },
    //     {
    //       "created_at": "2026-09-26T16:00:00Z",
    //       "events": [
    //         "deal.won"
    //       ],
    //       "id": "ebc7ef80-6cc7-5bcc-896f-309e9b0e282e",
    //       "is_active": true,
    //       "name": "Northwind Traders",
    //       "url": "https://erp.acme.example/crm-conversions"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "created_at": "2026-08-07T16:00:00Z",
      "events": [
        "contact.subscribed",
        "contact.unsubscribed"
      ],
      "id": "9a087430-47c9-5317-8abb-3f6fe7a9cbf9",
      "is_active": true,
      "name": "Northwind Traders",
      "url": "https://api.acme.example/hooks/crm"
    },
    {
      "created_at": "2026-09-26T16:00:00Z",
      "events": [
        "deal.won"
      ],
      "id": "ebc7ef80-6cc7-5bcc-896f-309e9b0e282e",
      "is_active": true,
      "name": "Northwind Traders",
      "url": "https://erp.acme.example/crm-conversions"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Webhook>(serde_json::json!({"created_at":"2026-08-07T16:00:00Z","events":["contact.subscribed","contact.unsubscribed"],"id":"9a087430-47c9-5317-8abb-3f6fe7a9cbf9","is_active":true,"name":"Northwind Traders","url":"https://api.acme.example/hooks/crm"})).unwrap_or_default(), serde_json::from_value::<Webhook>(serde_json::json!({"created_at":"2026-09-26T16:00:00Z","events":["deal.won"],"id":"ebc7ef80-6cc7-5bcc-896f-309e9b0e282e","is_active":true,"name":"Northwind Traders","url":"https://erp.acme.example/crm-conversions"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
