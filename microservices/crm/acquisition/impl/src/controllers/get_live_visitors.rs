// Implementation stub for handler 'get_live_visitors'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::get_live_visitors::{Request, Response};

#[handler(GetLiveVisitorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        anonymous_count: None, // TODO: Set from your business logic
        items: vec![serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:59:48Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","page":"/pricing","source":"email · oct-product-update"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:58:00Z","contact_id":"5de76105-f8c3-5357-af13-c5ce28753ed0","contact_name":"Jonas Weber","source":"app"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:54:00Z","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","contact_name":"Lena Fischer","page":"/launch","source":"google · cpc"}"#).unwrap_or(serde_json::Value::Null)], // TODO: Set from your business logic
    })
}
