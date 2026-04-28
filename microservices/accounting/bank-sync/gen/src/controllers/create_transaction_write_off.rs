// User-owned controller for handler 'create_transaction_write_off'.

use crate::handlers::create_transaction_write_off::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTransactionWriteOffController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        adjustment_type: "example".to_string(),
        amount: 3.14,
        id: "example".to_string(),
        journal_entry_id: Some("example".to_string()),
        transaction_id: "example".to_string(),
    }
}
