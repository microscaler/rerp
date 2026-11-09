// User-owned controller for handler 'get_outreach_overview'.

use crate::handlers::get_outreach_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetOutreachOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        anonymous_open_rate: Some(1.5),
        bounce_rate: Some(1.5),
        campaigns_sent: 42,
        click_rate: 1.5,
        click_to_open_rate: Some(1.5),
        messages_sent: 42,
        open_rate: 1.5,
        subscribed_contacts: Some(42),
        total_contacts: 42,
        unsubscribe_rate: Some(1.5),
    })
}
