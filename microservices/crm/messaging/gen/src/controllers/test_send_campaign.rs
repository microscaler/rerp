// User-owned controller for handler 'test_send_campaign'.

use crate::handlers::test_send_campaign::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestSendCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        message_ids: Some(vec![]),
        sent_to: vec![],
    })
}
