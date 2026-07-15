// User-owned controller for handler 'get_journal'.

use crate::handlers::get_journal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetJournalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: "example".to_string(),
        company_code: Some("example".to_string()),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_id: Some("example".to_string()),
        default_account_id: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        name: "example".to_string(),
        restrict_mode_hash_table: Some(true),
        sequence_number_next: Some(42),
        sequence_prefix: Some("example".to_string()),
        suspense_account_id: Some("example".to_string()),
        r#type: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
