// User-owned controller for handler 'ad_platform_webhook'.

use crate::handlers::ad_platform_webhook::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AdPlatformWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {})
}
