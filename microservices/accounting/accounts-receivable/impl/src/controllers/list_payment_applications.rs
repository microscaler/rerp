// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::list_payment_applications::Request;
use serde_json::Value;

#[handler(ListPaymentApplicationsController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let result = crate::http_support::with_ar_transaction(&context, |executor| {
        crate::applications::list_payment_applications(executor, &context, &request.data.id)
    });
    match result {
        Ok(applications) => HttpJson::new(200, applications),
        Err(error) => crate::http_support::ar_error(error),
    }
}
