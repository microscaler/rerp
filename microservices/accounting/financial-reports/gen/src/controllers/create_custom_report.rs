// User-owned controller for handler 'create_custom_report'.

use crate::handlers::create_custom_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCustomReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        definition: Some(Default::default()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        is_shared: Some(true),
        name: "example".to_string(),
        r#type: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
