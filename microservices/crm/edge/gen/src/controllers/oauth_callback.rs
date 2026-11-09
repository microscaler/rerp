// User-owned controller for handler 'oauth_callback'.

use crate::handlers::oauth_callback::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(OauthCallbackController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: 42,
        details: Some("example".to_string()),
        fields: Some(vec![]),
        message: "example".to_string(),
        reason: "example".to_string(),
        request_id: Some("example".to_string()),
    })
}
