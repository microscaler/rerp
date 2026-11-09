// Implementation stub for handler 'list_pipelines'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_pipelines::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::Pipeline;

#[handler(ListPipelinesController)]
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
        items: vec![serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"ROUND_ROBIN","closed_deals":7,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","name":"Sales","open_deals":38,"open_value":"412300.00","visibility":"ALL"})).unwrap_or_default(), serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"OFF","closed_deals":2,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"7bb3c22d-2ff5-5dd8-8252-93551ce02c7c","name":"Partners","open_deals":6,"open_value":"48000.00","visibility":"ALL"})).unwrap_or_default(), serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"OFF","closed_deals":2,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"0b83cac7-7b15-5d24-b8bd-bb5960b6bec6","name":"Renewals","open_deals":11,"open_value":"96500.00","visibility":"ALL"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
