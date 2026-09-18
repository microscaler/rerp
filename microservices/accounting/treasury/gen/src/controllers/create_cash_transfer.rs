// User-owned controller for handler 'create_cash_transfer'.

use crate::handlers::create_cash_transfer::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCashTransferController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        currency_code: Some("example".to_string()),
        destination_account_id: "example".to_string(),
        id: "example".to_string(),
        source_account_id: "example".to_string(),
        status: Default::default(),
    }
}
