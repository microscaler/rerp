// Implementation stub for handler 'list_dashboards'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::list_dashboards::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_insights_gen::handlers::types::Dashboard;

#[handler(ListDashboardsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Dashboard>(serde_json::json!({"created_at":"2026-10-26T16:00:00Z","id":"d882d72a-b5af-5bb5-bd44-13bca206fd5b","is_default":true,"name":"Alex's dashboard","updated_at":"2026-11-05T15:59:00Z","user_id":"alex","widgets":[{"id":"91bbb6c0-47bc-562d-9ecb-6bd37b17b013","kind":"NUMBERS","position":0,"query":{"metric":"email_kpis","range":"LAST_30_DAYS"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Key email marketing metrics","width":2},{"id":"c55b2ef3-e9dc-5b6f-8a2a-fd6ae58cd32b","kind":"BAR","position":1,"query":{"granularity":"MONTH","metric":"total_contacts","range":"LAST_12_MONTHS"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Contact growth over time","width":1},{"id":"dd78f406-2156-5e00-9f58-1c3cc539fc33","kind":"TABLE","position":2,"query":{"dimension":"automation","limit":5,"metric":"automation_entries","range":"ALL_TIME"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Most used automations by contacts entered","width":1}]})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 1, // TODO: Set from your business logic
    })
}
