// User-owned controller for handler 'list_webhook_logs'.

use crate::handlers::list_webhook_logs::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WebhookDeliveryLog;

#[handler(ListWebhookLogsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "delivered_at": "2026-11-05T16:00:00Z",
    //       "event": "Event",
    //       "response_body": "Response body",
    //       "response_status": 3,
    //       "retry_count": 3,
    //       "webhook_id": "7ba8e0e0-21a1-5f74-a91c-793dc622240e"
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
      "delivered_at": "2026-11-05T16:00:00Z",
      "event": "Event",
      "response_body": "Response body",
      "response_status": 3,
      "retry_count": 3,
      "webhook_id": "7ba8e0e0-21a1-5f74-a91c-793dc622240e"
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
        items: vec![serde_json::from_value::<WebhookDeliveryLog>(serde_json::json!({"delivered_at":"2026-11-05T16:00:00Z","event":"Event","response_body":"Response body","response_status":3,"retry_count":3,"webhook_id":"7ba8e0e0-21a1-5f74-a91c-793dc622240e"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
