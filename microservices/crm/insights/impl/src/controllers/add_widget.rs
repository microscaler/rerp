// Implementation stub for handler 'add_widget'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::add_widget::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::ReportSchedule;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::WidgetQuery;

#[handler(AddWidgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let kind = req.inner.kind;// let position = req.inner.position;// let prompt = req.inner.prompt;// let query = req.inner.query;// let title = req.inner.title;// let width = req.inner.width;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        id: "3d0b28d9-5ddf-5113-8e16-97b0bc878ccf".to_string(), // TODO: Set from your business logic
        kind: "TABLE".to_string(), // TODO: Set from your business logic
        position: None,            // TODO: Set from your business logic
        prompt: None,              // TODO: Set from your business logic
        query: serde_json::from_value::<WidgetQuery>(
            serde_json::json!({"dimension":"list","metric":"list_count","range":"LAST_30_DAYS"}),
        )
        .unwrap_or_default(), // TODO: Set from your business logic
        refreshed_at: None,        // TODO: Set from your business logic
        schedule: None,            // TODO: Set from your business logic
        title: "Contacts per list".to_string(), // TODO: Set from your business logic
        width: None,               // TODO: Set from your business logic
    })
}
