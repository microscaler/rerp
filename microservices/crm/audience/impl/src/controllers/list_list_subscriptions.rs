// Implementation stub for handler 'list_list_subscriptions'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_audience_gen::handlers::list_list_subscriptions::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_audience_gen::handlers::types::Subscription;

#[handler(ListListSubscriptionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;// let status = req.inner.status;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Subscription>(serde_json::json!({"contact_id":"07a7b393-a2b8-5f9d-86c2-37bc92d15398","created_at":"2026-11-05T16:00:00Z","id":"44f5daeb-faac-5a53-81fb-ac8b5f267d2c","list_id":"217107c2-57c1-5880-b574-99c272405f82","status":"PENDING"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
