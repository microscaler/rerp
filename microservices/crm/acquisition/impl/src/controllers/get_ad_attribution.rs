// Implementation stub for handler 'get_ad_attribution'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::get_ad_attribution::{Request, Response};

#[handler(GetAdAttributionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let model = req.inner.model;// let window_days = req.inner.window_days;// let by = req.inner.by;// let from = req.inner.from;// let to = req.inner.to;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        from: None,         // TODO: Set from your business logic
        model: None,        // TODO: Set from your business logic
        rows: None,         // TODO: Set from your business logic
        to: None,           // TODO: Set from your business logic
        totals: None,       // TODO: Set from your business logic
        warm_up_lift: None, // TODO: Set from your business logic
        window_days: None,  // TODO: Set from your business logic
    })
}
