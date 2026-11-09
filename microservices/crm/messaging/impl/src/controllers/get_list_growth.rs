// Implementation stub for handler 'get_list_growth'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::get_list_growth::{Request, Response};

#[handler(GetListGrowthController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let days = req.inner.days;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        list_id: "03493116-74f7-52f2-8ad2-d3daeb50dfc8".to_string(), // TODO: Set from your business logic
        points: vec![serde_json::from_str::<serde_json::Value>(
            r#"{"date":"2026-11-25","subscribed":3,"total":2,"unsubscribed":3}"#,
        )
        .unwrap_or(serde_json::Value::Null)], // TODO: Set from your business logic
    })
}
