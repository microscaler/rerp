// User-owned controller for handler 'list_chart_of_accounts'.

use crate::handlers::list_chart_of_accounts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ChartOfAccount;

#[handler(ListChartOfAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
