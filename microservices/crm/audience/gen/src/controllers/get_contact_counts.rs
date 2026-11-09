// User-owned controller for handler 'get_contact_counts'.

use crate::handlers::get_contact_counts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetContactCountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        bounced: Some(42),
        pending: Some(42),
        segments: Some(vec![]),
        subscribed: Some(42),
        total: 42,
        unsubscribed: Some(42),
    })
}
