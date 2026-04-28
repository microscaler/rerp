// User-owned controller for handler 'post_recognition_run'.

use crate::handlers::post_recognition_run::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PostRecognitionRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        fiscal_period_id: "example".to_string(),
        id: "example".to_string(),
        posted_journal_entry_id: Some("example".to_string()),
        status: Default::default(),
    }
}
