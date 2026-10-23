// User-owned controller for handler 'get_accounts_report'.

use crate::handlers::get_accounts_report::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountsReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        limit: 42,
        page: 42,
        total: 42,
    }
}
