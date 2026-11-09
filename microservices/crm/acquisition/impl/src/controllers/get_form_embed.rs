// Implementation stub for handler 'get_form_embed'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::get_form_embed::{Request, Response};

#[handler(GetFormEmbedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        form_id: "c49b4999-d0ed-58c1-a824-1ffe42b213e8".to_string(), // TODO: Set from your business logic
        hosted_url: "https://northwindtraders.example/pricing".to_string(), // TODO: Set from your business logic
        inline_html: None,                    // TODO: Set from your business logic
        script_tag: "Script tag".to_string(), // TODO: Set from your business logic
    })
}
