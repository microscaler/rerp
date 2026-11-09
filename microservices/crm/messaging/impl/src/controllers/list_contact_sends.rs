// Implementation stub for handler 'list_contact_sends'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_contact_sends::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::Send;

#[handler(ListContactSendsController)]
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
        items: vec![serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"7de39bcc-4bc6-551b-a967-7b1629b846be","campaign_name":"October product update","channel":"EMAIL","clicked_at":"2026-10-28T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-28T16:00:00Z","id":"540e3991-1a5a-53f5-b5d2-40847552e9b8","opened_at":"2026-10-28T16:00:00Z","status":"DELIVERED","subject":"October product update","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"cd847d34-2090-5e40-afc2-06c94055cc85","campaign_name":"Webinar invite · pricing strategy","channel":"EMAIL","clicked_at":"2026-11-05T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-08T16:00:00Z","id":"83e7787e-deb9-5ece-93c5-58284dd9ceb5","opened_at":"2026-10-08T16:00:00Z","status":"DELIVERED","subject":"Webinar invite · pricing strategy","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"dcad966f-24e6-5a17-bb05-807224d4fe09","campaign_name":"September product update","channel":"EMAIL","clicked_at":"2026-11-05T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-07-20T16:00:00Z","id":"f4276478-8eee-5e00-ae6b-d72dff226d41","opened_at":"2026-07-20T16:00:00Z","status":"DELIVERED","subject":"October product update","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
