// User-owned controller for handler 'create_workspace'.

use crate::handlers::create_workspace::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateWorkspaceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        brand_domain: Some("example".to_string()),
        created_at: Some("example".to_string()),
        default_currency: Some("example".to_string()),
        id: "example".to_string(),
        my_role: Some("example".to_string()),
        name: "example".to_string(),
        organisation_id: "example".to_string(),
        slug: Some("example".to_string()),
        timezone: Some("example".to_string()),
    })
}
