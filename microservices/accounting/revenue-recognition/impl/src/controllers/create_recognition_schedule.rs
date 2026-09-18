// Implementation stub for handler 'create_recognition_schedule'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_revenue_recognition_gen::handlers::create_recognition_schedule::{Request, Response};

#[handler(CreateRecognitionScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        currency_code: None,
        id: "".to_string(),
        recognized_amount: 0.0,
        rule_id: "".to_string(),
        source_invoice_id: "".to_string(),
        total_amount: 0.0,
    }
}
