// User-owned controller for handler 'create_social_post'.

use crate::handlers::create_social_post::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Media;
#[allow(unused_imports)]
use crate::handlers::types::PostStatus;
#[allow(unused_imports)]
use crate::handlers::types::PostTarget;

#[handler(CreateSocialPostController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        body: "example".to_string(),
        created_at: "example".to_string(),
        created_by: Some("example".to_string()),
        id: "example".to_string(),
        link_url: Some("example".to_string()),
        media: Some(vec![]),
        requires_approval: Some(true),
        scheduled_at: Some("example".to_string()),
        skip_reason: Some("example".to_string()),
        source: Some("example".to_string()),
        status: Default::default(),
        targets: vec![],
        updated_at: Some("example".to_string()),
        utm_campaign_id: Some("example".to_string()),
    })
}
