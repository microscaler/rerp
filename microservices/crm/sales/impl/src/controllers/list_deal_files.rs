// Implementation stub for handler 'list_deal_files'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_deal_files::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::Attachment;

#[handler(ListDealFilesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Attachment>(serde_json::json!({"created_at":"2026-11-04T11:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","file_name":"Northwind-proposal-v2.pdf","filename":"Filename","id":"d8f2aca9-2c33-5bb8-9b2a-3aba9a6b7ca6","size":3,"size_bytes":319_488,"uploaded_by":"Alex Morgan","url":"https://crm.acme.example/files/dfile1"})).unwrap_or_default(), serde_json::from_value::<Attachment>(serde_json::json!({"created_at":"2026-11-04T13:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","file_name":"SAML-setup-guide.pdf","filename":"Filename","id":"9c808bcb-a18d-53f5-9805-02247a239a42","size":3,"size_bytes":1_153_433,"uploaded_by":"Alex Morgan","url":"https://crm.acme.example/files/dfile2"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
