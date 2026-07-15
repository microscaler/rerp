// User-owned controller for handler 'delete_journal'.

use crate::handlers::delete_journal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteJournalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    })
}
