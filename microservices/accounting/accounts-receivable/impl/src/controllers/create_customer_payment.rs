// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::create_customer_payment::Request;
use serde_json::Value;

#[handler(CreateCustomerPaymentController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let query = &request.data;
    let auto_apply = query.auto_apply.unwrap_or(false);
    let result = crate::http_support::with_ar_transaction(&context, |executor| {
        let payment = crate::payments::create_payment(
            executor,
            &context,
            &query.amount,
            &query.currency_code,
            &query.payment_method,
            &query.payment_date,
            &query.customer_id,
            query.company_id.as_deref(),
            query.reference_number.as_deref(),
            query.notes.as_deref(),
        )?;
        if auto_apply {
            // FIFO allocation over the customer's open items, contract axis.
            crate::applications::auto_apply_fifo(
                executor,
                &context,
                payment["id"].as_str().unwrap_or_default(),
            )
        } else {
            Ok(payment)
        }
    });
    match result {
        Ok(payment) => HttpJson::new(201, payment),
        Err(error) => crate::http_support::ar_error(error),
    }
}
