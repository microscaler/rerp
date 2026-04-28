// User-owned controller for handler 'get_custom_report'.

use crate::handlers::get_custom_report::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCustomReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
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
    }
}
