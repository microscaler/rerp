// Implementation stub for handler 'list_job_errors'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_job_errors::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::JobError;

#[handler(ListJobErrorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<JobError>(serde_json::json!({"code":"INVALID_EMAIL","field":"email","message":"\"jane@@example\" is not a valid address","raw":{"Company":"Acme","Email":"jane@@example","First":"Jane","Last":"Doe"},"row":14})).unwrap_or_default(), serde_json::from_value::<JobError>(serde_json::json!({"code":"DUPLICATE","field":"email","message":"Row 12 has the same email; dedupe on EMAIL kept the first","raw":{"Email":"priya.natarajan@northwindtraders.example","First":"P","Last":"Natarajan"},"row":88})).unwrap_or_default(), serde_json::from_value::<JobError>(serde_json::json!({"code":"UNKNOWN_FIELD","field":"plan","message":"Column \"plan\" was not mapped to a field","raw":{},"row":203})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
