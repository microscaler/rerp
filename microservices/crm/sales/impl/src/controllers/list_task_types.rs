// Implementation stub for handler 'list_task_types'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_task_types::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::TaskType;

#[handler(ListTaskTypesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let active = req.inner.active;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":15,"id":"4eec3526-5869-5e76-820f-ac2358c6b899","open_count":12,"outcomes":["Connected","No answer","Left voicemail"],"title":"Call"})).unwrap_or_default(), serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":0,"id":"099104fc-dcee-57bf-b906-d2b826beb51d","open_count":8,"outcomes":["Sent","Replied","Bounced"],"title":"Email"})).unwrap_or_default(), serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":45,"id":"645bfe17-69c3-558e-82cd-9594c3609fa2","open_count":5,"outcomes":["Held","No-show","Rescheduled"],"title":"Meeting"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
