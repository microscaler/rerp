// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::list_open_items::Request;
use serde_json::Value;

#[handler(ListOpenItemsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let filters = match crate::ledger::parse_list_filters(
        query.status.as_deref(),
        query.aging_bucket.as_deref(),
        query.customer_id.as_deref(),
        query.company_id.as_deref(),
        query.currency_code.as_deref(),
        query.days_overdue_min,
        query.days_overdue_max,
        query.search.as_deref(),
        query.page,
        query.limit,
    ) {
        Ok(filters) => filters,
        Err(error) => return crate::http_support::ar_error(error),
    };
    match crate::http_support::with_ar_transaction(&context, |executor| {
        crate::ledger::list_open_items(executor, &context, &filters)
    }) {
        Ok(items) => HttpJson::new(200, items),
        Err(error) => crate::http_support::ar_error(error),
    }
}
