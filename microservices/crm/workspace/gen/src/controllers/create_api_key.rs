// User-owned controller for handler 'create_api_key'.

use crate::handlers::create_api_key::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateApiKeyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        id: Some("example".to_string()),
        key: Some("example".to_string()),
        key_hash: Some("example".to_string()),
        name: Some("example".to_string()),
    })
}
