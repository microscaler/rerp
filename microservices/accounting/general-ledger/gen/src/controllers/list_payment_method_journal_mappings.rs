// User-owned controller for handler 'list_payment_method_journal_mappings'.

use crate::handlers::list_payment_method_journal_mappings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PaymentMethodJournalMapping;

#[handler(ListPaymentMethodJournalMappingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response(vec![]))
}
