// User-owned controller for handler 'upload_deal_file'.

use crate::handlers::upload_deal_file::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UploadDealFileController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        content_type: Some("example".to_string()),
        created_at: "example".to_string(),
        filename: "example".to_string(),
        id: "example".to_string(),
        size: 42,
        uploaded_by: Some("example".to_string()),
        url: Some("example".to_string()),
    })
}
