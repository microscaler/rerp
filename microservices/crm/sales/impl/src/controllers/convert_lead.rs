// Implementation stub for handler 'convert_lead'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::convert_lead::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_sales_gen::handlers::types::ConversionCustomer;

#[handler(ConvertLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let customer = req.inner.customer;// let reason = req.inner.reason;// let reference = req.inner.reference;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let idempotency_key = req.inner.idempotency_key;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        conversion_id: "2c4a2628-c1ef-51f9-bb85-7af366714c87".to_string(), // TODO: Set from your business logic
        lead_id: "36ad9027-d2b6-5440-962c-1bf60325b115".to_string(), // TODO: Set from your business logic
        message_id: None,            // TODO: Set from your business logic
        reason: "order".to_string(), // TODO: Set from your business logic
        stage: "Stage".to_string(),  // TODO: Set from your business logic
    })
}
