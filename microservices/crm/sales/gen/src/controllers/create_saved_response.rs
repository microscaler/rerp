// User-owned controller for handler 'create_saved_response'.

use crate::handlers::create_saved_response::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateSavedResponseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        body: "example".to_string(),
        category: Some("example".to_string()),
        category_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        id: "example".to_string(),
        last_sent_at: Some("example".to_string()),
        last_sent_by: Some("example".to_string()),
        name: "example".to_string(),
        send_count: Some(42),
        subject: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
