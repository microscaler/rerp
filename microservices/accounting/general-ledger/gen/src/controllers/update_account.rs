// User-owned controller for handler 'update_account'.

use crate::handlers::update_account::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        account_type: "example".to_string(),
        chart_of_account_id: "example".to_string(),
        code: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "example".to_string(),
        description: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        is_system_account: true,
        metadata: Some(Default::default()),
        name: "example".to_string(),
        normal_balance: "example".to_string(),
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
    }
}
