// User-owned controller for handler 'ad_platform_webhook'.

use crate::handlers::ad_platform_webhook::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AdPlatformWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: 42,
        details: Some("example".to_string()),
        fields: Some(vec![]),
        message: "example".to_string(),
        request_id: Some("example".to_string()),
    })
}
