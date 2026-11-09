// Implementation stub for handler 'list_deal_activity'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_deal_activity::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::ActivityEntry;

#[handler(ListDealActivityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let kind = req.inner.kind;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"mailbox","at":"2026-11-05T11:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"3e3541c5-986e-5136-bf06-7fdcf58b3b41","kind":"EMAIL_RECEIVED","summary":"Email received from Priya Natarajan"})).unwrap_or_default(), serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"Alex Morgan","at":"2026-11-04T14:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"56f9f35a-b3ae-561e-a01d-fc17a886c66f","kind":"CREATED","summary":"Note added"})).unwrap_or_default(), serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"Alex Morgan","at":"2026-11-04T11:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"49dfa063-a2e2-5b6b-8052-24d0928bc8d4","kind":"EMAIL_SENT","summary":"1:1 email sent · Proposal for 120 seats"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 6, // TODO: Set from your business logic
    })
}
