// User-owned controller for handler 'auto_match_transactions'.

use crate::handlers::auto_match_transactions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AutoMatchTransactionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        matched_count: Some(42),
        matches: Some(vec![]),
        remaining_unmatched: Some(42),
    })
}
