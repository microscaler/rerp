// Implementation stub for handler 'list_deal_notes'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_deal_notes::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::Note;

#[handler(ListDealNotesController)]
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
        items: vec![serde_json::from_value::<Note>(serde_json::json!({"author":"Alex Morgan","created_at":"2026-11-04T14:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"c5a53e54-861c-5955-9c05-355ff0f0ab0c","text":"Priya confirmed budget approved for 120 seats; SSO is the blocker. Sent SAML docs."})).unwrap_or_default(), serde_json::from_value::<Note>(serde_json::json!({"author":"Automation · Hot lead handoff","created_at":"2026-11-02T09:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"938887ff-e5fb-5b68-b294-071e744f7951","text":"Deal created from form \"Contact sales\" (seats = 120)."})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
