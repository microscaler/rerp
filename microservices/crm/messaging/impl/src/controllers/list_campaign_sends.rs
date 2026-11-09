// Implementation stub for handler 'list_campaign_sends'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_campaign_sends::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::Send;

#[handler(ListCampaignSendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;// let status = req.inner.status;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","clicked_link":"changelog","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","created_at":"2026-11-05T16:00:00Z","id":"01a1ef84-be9a-5166-aaa5-6a6a9902308c","opened_at":"2026-10-28T16:00:00Z","status":"DELIVERED","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","clicked_link":"docs/imports","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","contact_name":"Tom Okafor","created_at":"2026-11-05T16:00:00Z","id":"294a11f7-a7ec-5743-be57-1a1f486a17b9","opened_at":"2026-10-28T17:00:00Z","status":"DELIVERED","to":"tom.okafor@fabrikam.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","clicked_link":"Clicked link","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","contact_name":"Lena Fischer","created_at":"2026-11-05T16:00:00Z","id":"c84b4b32-0560-5476-99cf-1822e1da82fe","opened_at":"2026-11-05T16:00:00Z","status":"BOUNCED","to":"lena.fischer@contoso.example"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 9870, // TODO: Set from your business logic
    })
}
