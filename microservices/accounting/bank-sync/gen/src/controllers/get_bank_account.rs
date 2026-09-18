// User-owned controller for handler 'get_bank_account'.

use crate::handlers::get_bank_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetBankAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_number: "example".to_string(),
        account_type: Some("example".to_string()),
        bank_id: Some("example".to_string()),
        bank_name: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        gl_account_id: Some("example".to_string()),
        id: "example".to_string(),
        last_sync_date: Some("example".to_string()),
        name: "example".to_string(),
        notes: Some("example".to_string()),
        opening_balance: Some(3.14),
        opening_balance_date: Some("example".to_string()),
        routing_number: Some("example".to_string()),
        status: "example".to_string(),
        sync_enabled: Some(true),
        sync_frequency: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    })
}
