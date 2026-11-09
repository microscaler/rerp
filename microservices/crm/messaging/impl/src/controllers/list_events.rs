// Implementation stub for handler 'list_events'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_events::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::MessageEvent;

#[handler(ListEventsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let contact_id = req.inner.contact_id;// let campaign_id = req.inner.campaign_id;// let send_id = req.inner.send_id;// let type = req.inner.type;// let since = req.inner.since;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<MessageEvent>(serde_json::json!({"at":"2026-11-05T16:00:00Z","contact_id":"c891a339-996d-56b9-a022-244c570f48c9","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","send_id":"a28a1db3-cee2-594e-adbd-25b5f5f34365","type":"QUEUED"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
