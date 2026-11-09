// Implementation stub for handler 'list_notification_rules'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_notification_rules::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::NotificationRule;

#[handler(ListNotificationRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Email + chat #sales-inbound","id":"f3473159-784b-56cd-8dc1-c540b9842e88","notify":"Sales team","when":"Form submitted · Contact sales"})).unwrap_or_default(), serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Pager channel","id":"f20c3651-3e1e-546f-862f-4a18c3982ea7","notify":"Marketing admins","when":"Campaign send failed or paused by provider"})).unwrap_or_default(), serde_json::from_value::<NotificationRule>(serde_json::json!({"enabled":true,"event":"Event","how":"Pager channel","id":"484181c6-2fee-52f2-b83e-bd0dc2669dd8","notify":"Marketing admins","when":"Complaint rate above 0.1% in 24 h"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 6, // TODO: Set from your business logic
    })
}
