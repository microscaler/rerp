// Implementation stub for handler 'create_cash_transfer'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_treasury_gen::handlers::create_cash_transfer::{Request, Response};

#[handler(CreateCashTransferController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        amount: 0.0,
        currency_code: None,
        destination_account_id: "".to_string(),
        id: "".to_string(),
        source_account_id: "".to_string(),
        status: serde_json::json!({}),
    }
}
