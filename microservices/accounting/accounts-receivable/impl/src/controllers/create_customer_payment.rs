// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::create_customer_payment::Request;
use serde_json::Value;

#[handler(CreateCustomerPaymentController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_write_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let result = crate::http_support::with_ar_transaction(&context, |executor| {
        // `auto_apply` rides into the command itself (and into its idempotency
        // fingerprint): the FIFO pass runs only on a fresh creation, never
        // on a replay that would silently consume newly opened items.
        crate::payments::create_payment(
            executor,
            &context,
            &query.idempotency_key,
            &query.amount,
            &query.currency_code,
            &query.payment_method,
            &query.payment_date,
            &query.customer_id,
            query.company_id.as_deref(),
            query.reference_number.as_deref(),
            query.notes.as_deref(),
            query.auto_apply.unwrap_or(false),
        )
    });
    match result {
        Ok(payment) => HttpJson::new(201, payment),
        Err(error) => crate::http_support::ar_error(error),
    }
}
