// User-owned controller for handler 'create_transaction_exchange_difference'.

use crate::handlers::create_transaction_exchange_difference::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTransactionExchangeDifferenceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        adjustment_type: "example".to_string(),
        amount: 3.14,
        id: "example".to_string(),
        journal_entry_id: Some("example".to_string()),
        transaction_id: "example".to_string(),
    })
}
