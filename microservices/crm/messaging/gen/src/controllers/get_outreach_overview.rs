// User-owned controller for handler 'get_outreach_overview'.

use crate::handlers::get_outreach_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetOutreachOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "bounce_rate": 0.009,
    //   "campaigns_sent": 6,
    //   "click_rate": 0.046,
    //   "messages_sent": 61402,
    //   "open_rate": 0.382,
    //   "subscribed_contacts": 10912,
    //   "total_contacts": 12480,
    //   "unsubscribe_rate": 0.003
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "bounce_rate": 0.009,
  "campaigns_sent": 6,
  "click_rate": 0.046,
  "messages_sent": 61402,
  "open_rate": 0.382,
  "subscribed_contacts": 10912,
  "total_contacts": 12480,
  "unsubscribe_rate": 0.003
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        anonymous_open_rate: Some(1.5),
        bounce_rate: Some(0.009),
        campaigns_sent: 6,
        click_rate: 0.046,
        click_to_open_rate: Some(1.5),
        messages_sent: 61402,
        open_rate: 0.382,
        subscribed_contacts: Some(10912),
        total_contacts: 12480,
        unsubscribe_rate: Some(0.003),
    })
}
