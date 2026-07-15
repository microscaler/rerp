// User-owned controller for handler 'create_chart_of_account'.

use crate::handlers::create_chart_of_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_type: "example".to_string(),
        code: "example".to_string(),
        created_at: Some("example".to_string()),
        description: "example".to_string(),
        id: "example".to_string(),
        is_active: true,
        level: 42,
        name: "example".to_string(),
        parent_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    })
}
