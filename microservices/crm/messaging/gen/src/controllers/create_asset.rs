// User-owned controller for handler 'create_asset'.

use crate::handlers::create_asset::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Asset;

#[handler(CreateAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        asset: Default::default(),
        expires_at: "example".to_string(),
        upload_url: "example".to_string(),
    })
}
