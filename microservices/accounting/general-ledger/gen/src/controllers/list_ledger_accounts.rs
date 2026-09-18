// User-owned controller for handler 'list_ledger_accounts'.

use crate::handlers::list_ledger_accounts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LedgerAccount;

#[handler(ListLedgerAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        limit: 42,
    })
}
