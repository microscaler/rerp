// Implementation stub for handler 'list_unassigned'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_unassigned::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::UnassignedLead;

#[handler(ListUnassignedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let team_id = req.inner.team_id;// let stage = req.inner.stage;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<UnassignedLead>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","email_from":"Email from","lead_id":"36ad9027-d2b6-5440-962c-1bf60325b115","lead_name":"Lead name","source":"Source"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
