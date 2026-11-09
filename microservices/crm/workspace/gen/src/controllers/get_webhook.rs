// User-owned controller for handler 'get_webhook'.

use crate::handlers::get_webhook::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WebhookDeliveryStatus;

#[handler(GetWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        events: vec![],
        headers: Some(Default::default()),
        id: "example".to_string(),
        is_active: true,
        last_status: Some(Default::default()),
        last_triggered: Some("example".to_string()),
        name: "example".to_string(),
        retry_count: Some(42),
        secret: Some("example".to_string()),
        url: "example".to_string(),
    })
}
