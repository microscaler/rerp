// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::create_payment_application::Request;
use serde_json::Value;

#[handler(CreatePaymentApplicationController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_write_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let result = crate::http_support::with_ar_transaction(&context, |executor| {
        crate::applications::create_payment_application(
            executor,
            &context,
            &query.payment_id,
            &query.customer_invoice_id,
            &query.applied_amount,
        )
    });
    match result {
        Ok(application) => HttpJson::new(201, application),
        Err(error) => crate::http_support::ar_error(error),
    }
}
