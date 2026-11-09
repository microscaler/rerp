// User-owned controller for handler 'list_notification_rules'.

use crate::handlers::list_notification_rules::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NotificationRule;

#[handler(ListNotificationRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "enabled": true,
    //       "event": "Event",
    //       "how": "Email + chat #sales-inbound",
    //       "id": "f3473159-784b-56cd-8dc1-c540b9842e88",
    //       "notify": "Sales team",
    //       "when": "Form submitted · Contact sales"
    //     },
    //     {
    //       "enabled": true,
    //       "event": "Event",
    //       "how": "Pager channel",
    //       "id": "f20c3651-3e1e-546f-862f-4a18c3982ea7",
    //       "notify": "Marketing admins",
    //       "when": "Campaign send failed or paused by provider"
    //     },
    //     {
    //       "enabled": true,
    //       "event": "Event",
    //       "how": "Pager channel",
    //       "id": "484181c6-2fee-52f2-b83e-bd0dc2669dd8",
    //       "notify": "Marketing admins",
    //       "when": "Complaint rate above 0.1% in 24 h"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 6
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "enabled": true,
      "event": "Event",
      "how": "Email + chat #sales-inbound",
      "id": "f3473159-784b-56cd-8dc1-c540b9842e88",
      "notify": "Sales team",
      "when": "Form submitted · Contact sales"
    },
    {
      "enabled": true,
      "event": "Event",
      "how": "Pager channel",
      "id": "f20c3651-3e1e-546f-862f-4a18c3982ea7",
      "notify": "Marketing admins",
      "when": "Campaign send failed or paused by provider"
    },
    {
      "enabled": true,
      "event": "Event",
      "how": "Pager channel",
      "id": "484181c6-2fee-52f2-b83e-bd0dc2669dd8",
      "notify": "Marketing admins",
      "when": "Complaint rate above 0.1% in 24 h"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 6
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Email + chat #sales-inbound","id":"f3473159-784b-56cd-8dc1-c540b9842e88","notify":"Sales team","when":"Form submitted · Contact sales"})).unwrap_or_default(), serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Pager channel","id":"f20c3651-3e1e-546f-862f-4a18c3982ea7","notify":"Marketing admins","when":"Campaign send failed or paused by provider"})).unwrap_or_default(), serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Pager channel","id":"484181c6-2fee-52f2-b83e-bd0dc2669dd8","notify":"Marketing admins","when":"Complaint rate above 0.1% in 24 h"})).unwrap_or_default()],limit: 25,page: 1,total: 6,
    })
}
