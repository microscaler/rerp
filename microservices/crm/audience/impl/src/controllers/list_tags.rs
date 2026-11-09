// Implementation stub for handler 'list_tags'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_audience_gen::handlers::list_tags::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_audience_gen::handlers::types::Tag;

#[handler(ListTagsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Tag>(serde_json::json!({"color":"#1F5AF6","contact_count":12,"created_at":"2026-04-19T16:00:00Z","id":"8a4e41d9-bc47-5357-abf0-fe040418ee6e","name":"customer"})).unwrap_or_default(), serde_json::from_value::<Tag>(serde_json::json!({"color":"#0B7A4B","contact_count":6,"created_at":"2026-04-19T16:00:00Z","id":"1d7a712a-45bb-51db-8b14-23fb2e8296ff","name":"trial"})).unwrap_or_default(), serde_json::from_value::<Tag>(serde_json::json!({"color":"#B45309","contact_count":3,"created_at":"2026-04-19T16:00:00Z","id":"c0e89ee1-beaa-54aa-8af4-f16c1bdb152b","name":"waitlist"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 12, // TODO: Set from your business logic
    })
}
