// User-owned controller for handler 'update_notification_rule'.

use crate::handlers::update_notification_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateNotificationRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        channels: Some(vec![]),
        enabled: true,
        event: "example".to_string(),
        filter: Some(Default::default()),
        how: Some("example".to_string()),
        id: "example".to_string(),
        notify: Some("example".to_string()),
        recipients: Some(Default::default()),
        when: Some("example".to_string()),
    })
}
