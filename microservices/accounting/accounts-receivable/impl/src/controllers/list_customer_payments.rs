// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::list_customer_payments::Request;
use serde_json::Value;

#[handler(ListCustomerPaymentsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let filters = match crate::payments::parse_payment_list_filters(
        query.status.as_deref(),
        query.payment_method.as_deref(),
        query.customer_id.as_deref(),
        query.company_id.as_deref(),
        query.date_from.as_deref(),
        query.date_to.as_deref(),
        query.search.as_deref(),
        query.page,
        query.limit,
    ) {
        Ok(filters) => filters,
        Err(error) => return crate::http_support::ar_error(error),
    };
    match crate::http_support::with_ar_transaction(&context, |executor| {
        crate::payments::list_payments(executor, &context, &filters)
    }) {
        Ok(payments) => HttpJson::new(200, payments),
        Err(error) => crate::http_support::ar_error(error),
    }
}
