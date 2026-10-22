// User-owned controller for handler 'send_customer_statements'.

use crate::handlers::send_customer_statements::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SendCustomerStatementsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        id: "example".to_string(),
        statement_count: Some(42),
        status: "example".to_string(),
    })
}
