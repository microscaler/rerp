// User-owned controller for handler 'get_ad_account'.

use crate::handlers::get_ad_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        authorize_url: Some("example".to_string()),
        capabilities: Some(vec![]),
        connected_by: Some("example".to_string()),
        conversion_endpoint: Some("example".to_string()),
        created_at: Some("example".to_string()),
        default_currency: Some("example".to_string()),
        external_account_id: Some("example".to_string()),
        id: "example".to_string(),
        last_error: Some("example".to_string()),
        last_spend_import_at: Some("example".to_string()),
        last_sync_at: Some("example".to_string()),
        match_rate_last: Some(1.5),
        name: "example".to_string(),
        pixel_id: Some("example".to_string()),
        platform: "example".to_string(),
        status: "example".to_string(),
    })
}
