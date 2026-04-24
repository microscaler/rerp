// User-owned controller for handler 'create_fiscal_position'.

use crate::handlers::create_fiscal_position::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFiscalPositionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        auto_apply: Some(true),
        code: "example".to_string(),
        company_id: "example".to_string(),
        country_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        name: "example".to_string(),
        state_ids: Some(vec![]),
        updated_at: Some("example".to_string()),
    }
}
