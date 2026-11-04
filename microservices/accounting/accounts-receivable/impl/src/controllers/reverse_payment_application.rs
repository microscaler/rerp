// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::reverse_payment_application::Request;
use serde_json::Value;

#[handler(ReversePaymentApplicationController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let result = crate::http_support::with_ar_transaction(&context, |executor| {
        crate::applications::reverse_payment_application(
            executor,
            &context,
            &query.id,
            &query.idempotency_key,
            &query.reason,
        )
    });
    match result {
        Ok(payment) => HttpJson::new(200, payment),
        Err(error) => crate::http_support::ar_error(error),
    }
}
