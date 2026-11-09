// User-owned controller for handler 'create_ad_creative'.

use crate::handlers::create_ad_creative::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAdCreativeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        asset_id: Some("example".to_string()),
        body: Some("example".to_string()),
        brand_kit_applied: Some(true),
        call_to_action: Some("example".to_string()),
        created_at: Some("example".to_string()),
        duration_seconds: Some(42),
        headline: Some("example".to_string()),
        height: Some(42),
        id: "example".to_string(),
        kind: "example".to_string(),
        landing_url: Some("example".to_string()),
        name: "example".to_string(),
        platforms: Some(vec![]),
        url: Some("example".to_string()),
        width: Some(42),
    })
}
