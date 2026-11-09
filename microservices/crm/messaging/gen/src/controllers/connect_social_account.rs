// User-owned controller for handler 'connect_social_account'.

use crate::handlers::connect_social_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Platform;

#[handler(ConnectSocialAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        authorization_url: Some("example".to_string()),
        avatar_url: Some("example".to_string()),
        created_at: "example".to_string(),
        display_name: Some("example".to_string()),
        followers: Some(42),
        handle: "example".to_string(),
        id: "example".to_string(),
        paused_reason: Some("example".to_string()),
        platform: Default::default(),
        status: "example".to_string(),
        token_expires_at: Some("example".to_string()),
    })
}
