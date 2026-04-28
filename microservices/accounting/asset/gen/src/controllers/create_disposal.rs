// User-owned controller for handler 'create_disposal'.

use crate::handlers::create_disposal::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDisposalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        approved_by: Some("example".to_string()),
        asset_id: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        description: Some("example".to_string()),
        disposal_date: "example".to_string(),
        disposal_type: "example".to_string(),
        gain_loss: Some(3.14),
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        net_book_value: 3.14,
        proceeds: Some(3.14),
    }
}
