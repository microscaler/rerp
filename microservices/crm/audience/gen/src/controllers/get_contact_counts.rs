// User-owned controller for handler 'get_contact_counts'.

use crate::handlers::get_contact_counts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetContactCountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "bounced": 464,
    //   "pending": 476,
    //   "subscribed": 10912,
    //   "total": 12480,
    //   "unsubscribed": 1104
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "bounced": 464,
  "pending": 476,
  "subscribed": 10912,
  "total": 12480,
  "unsubscribed": 1104
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        bounced: Some(464),
        pending: Some(476),
        segments: Some(vec![]),
        subscribed: Some(10912),
        total: 12480,
        unsubscribed: Some(1104),
    })
}
