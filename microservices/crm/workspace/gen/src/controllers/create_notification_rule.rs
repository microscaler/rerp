// User-owned controller for handler 'create_notification_rule'.

use crate::handlers::create_notification_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateNotificationRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "enabled": true,
    //   "event": "Event",
    //   "how": "Email + chat #sales-inbound",
    //   "id": "f3473159-784b-56cd-8dc1-c540b9842e88",
    //   "notify": "Sales team",
    //   "when": "Form submitted · Contact sales"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "enabled": true,
  "event": "Event",
  "how": "Email + chat #sales-inbound",
  "id": "f3473159-784b-56cd-8dc1-c540b9842e88",
  "notify": "Sales team",
  "when": "Form submitted · Contact sales"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        channels: Some(vec![]),
        enabled: true,
        event: "Event".to_string(),
        filter: Some(Default::default()),
        how: Some("Email + chat #sales-inbound".to_string()),
        id: "f3473159-784b-56cd-8dc1-c540b9842e88".to_string(),
        notify: Some("Sales team".to_string()),
        recipients: Some(Default::default()),
        when: Some("Form submitted · Contact sales".to_string()),
    })
}
