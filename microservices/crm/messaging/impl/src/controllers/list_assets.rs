// Implementation stub for handler 'list_assets'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_assets::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::Asset;

#[handler(ListAssetsController)]
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
        items: vec![serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","height":600,"id":"8692f757-7ab4-5792-8895-3cb42594a05d","name":"hero-october.png","size_bytes":412000,"url":"https://mail.acme.example/assets/hero-october.png","width":1200})).unwrap_or_default(), serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-10-27T16:00:00Z","height":600,"id":"d016ee7e-b764-5145-80a9-19a23ccce65e","name":"logo.png","size_bytes":18000,"url":"https://mail.acme.example/assets/logo.png","width":1200})).unwrap_or_default(), serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-10-18T16:00:00Z","height":600,"id":"83ff295d-5bfe-5dc1-acfe-3b615507475d","name":"case-study-northwind.jpg","size_bytes":630000,"url":"https://mail.acme.example/assets/case-study-northwind.jpg","width":1200})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 100, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 42, // TODO: Set from your business logic
    })
}
