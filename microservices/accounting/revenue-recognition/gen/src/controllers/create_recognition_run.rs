// User-owned controller for handler 'create_recognition_run'.

use crate::handlers::create_recognition_run::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateRecognitionRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        fiscal_period_id: "example".to_string(),
        id: "example".to_string(),
        posted_journal_entry_id: Some("example".to_string()),
        status: Default::default(),
    }
}
