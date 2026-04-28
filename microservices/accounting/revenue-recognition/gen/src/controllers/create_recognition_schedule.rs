// User-owned controller for handler 'create_recognition_schedule'.

use crate::handlers::create_recognition_schedule::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateRecognitionScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        currency_code: Some("example".to_string()),
        id: "example".to_string(),
        recognized_amount: 3.14,
        rule_id: "example".to_string(),
        source_invoice_id: "example".to_string(),
        total_amount: 3.14,
    }
}
