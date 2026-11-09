// Implementation stub for handler 'list_webhooks'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_webhooks::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::Webhook;

#[handler(ListWebhooksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let active = req.inner.active;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Webhook>(serde_json::json!({"created_at":"2026-08-07T16:00:00Z","events":["contact.subscribed","contact.unsubscribed"],"failures":0,"id":"9a087430-47c9-5317-8abb-3f6fe7a9cbf9","is_active":true,"name":"Northwind Traders","status":"OK","url":"https://api.acme.example/hooks/crm"})).unwrap_or_default(), serde_json::from_value::<Webhook>(serde_json::json!({"created_at":"2026-09-26T16:00:00Z","detail":"Signed · Pub/Sub","events":["deal.won"],"failures":0,"id":"ebc7ef80-6cc7-5bcc-896f-309e9b0e282e","is_active":true,"name":"Northwind Traders","status":"OK","url":"https://erp.acme.example/crm-conversions"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
