// User-owned controller for handler 'send_deal_email'.

use crate::handlers::send_deal_email::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SendDealEmailController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "at": "2026-11-05T16:00:00Z",
    //   "direction": "OUT",
    //   "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //   "subject": "October product update"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "at": "2026-11-05T16:00:00Z",
  "direction": "OUT",
  "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
  "subject": "October product update"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        at: "2026-11-05T16:00:00Z".to_string(),
        body_html: Some("example".to_string()),
        clicked_at: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        direction: "OUT".to_string(),
        from: Some("example".to_string()),
        id: "63a6661f-5860-50ed-8697-f28d29a3917f".to_string(),
        mailbox_id: Some("example".to_string()),
        opened_at: Some("example".to_string()),
        replied_at: Some("example".to_string()),
        saved_response_id: Some("example".to_string()),
        snippet: Some("example".to_string()),
        subject: "October product update".to_string(),
        thread_id: Some("example".to_string()),
        to: Some(vec![]),
    })
}
