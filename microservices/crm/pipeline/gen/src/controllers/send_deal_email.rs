// User-owned controller for handler 'send_deal_email'.

use crate::handlers::send_deal_email::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SendDealEmailController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        at: "example".to_string(),
        body_html: Some("example".to_string()),
        clicked_at: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        direction: "example".to_string(),
        from: Some("example".to_string()),
        id: "example".to_string(),
        mailbox_id: Some("example".to_string()),
        opened_at: Some("example".to_string()),
        replied_at: Some("example".to_string()),
        saved_response_id: Some("example".to_string()),
        snippet: Some("example".to_string()),
        subject: "example".to_string(),
        thread_id: Some("example".to_string()),
        to: Some(vec![]),
    })
}
