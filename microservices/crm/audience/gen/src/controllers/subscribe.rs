// User-owned controller for handler 'subscribe'.

use crate::handlers::subscribe::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConsentEvent;
#[allow(unused_imports)]
use crate::handlers::types::ConsentSource;
#[allow(unused_imports)]
use crate::handlers::types::SubscriptionStatus;

#[handler(SubscribeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        consent_at: Some("example".to_string()),
        consent_ip: Some("example".to_string()),
        consent_source: Some(Default::default()),
        consent_text: Some("example".to_string()),
        contact_id: "example".to_string(),
        created_at: "example".to_string(),
        form_id: Some("example".to_string()),
        history: Some(vec![]),
        id: "example".to_string(),
        list_id: "example".to_string(),
        list_name: Some("example".to_string()),
        status: Default::default(),
        unsubscribe_reason: Some("example".to_string()),
        unsubscribed_at: Some("example".to_string()),
    })
}
