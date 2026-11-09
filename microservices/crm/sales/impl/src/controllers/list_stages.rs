// Implementation stub for handler 'list_stages'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_stages::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::Stage;

#[handler(ListStagesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let team_id = req.inner.team_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#C9CFD8","fold":false,"id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","is_lost":false,"is_won":false,"name":"New","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":10,"rotting_days":14,"sequence":1,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default(), serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#9DB6F7","fold":false,"id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","is_lost":false,"is_won":false,"name":"Qualified","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":25,"rotting_days":14,"sequence":2,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default(), serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#1F5AF6","fold":false,"id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","is_lost":false,"is_won":false,"name":"Proposal","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"rotting_days":14,"sequence":3,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 16, // TODO: Set from your business logic
    })
}
