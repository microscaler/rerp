// Implementation stub for handler 'create_tracked_site'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::create_tracked_site::{Request, Response};

#[handler(CreateTrackedSiteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let name = req.inner.name;// let origin = req.inner.origin;// let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        created_at: "2026-05-09T16:00:00Z".to_string(), // TODO: Set from your business logic
        enabled: None,                                  // TODO: Set from your business logic
        events_24h: None,                               // TODO: Set from your business logic
        id: "f3a558f4-397d-5377-8279-803f527fe211".to_string(), // TODO: Set from your business logic
        identified_30d: None,    // TODO: Set from your business logic
        include_all_pages: None, // TODO: Set from your business logic
        kind: None,              // TODO: Set from your business logic
        last_event_at: None,     // TODO: Set from your business logic
        name: None,              // TODO: Set from your business logic
        origin: "https://acme.example".to_string(), // TODO: Set from your business logic
        snippet: None,           // TODO: Set from your business logic
        tracking_key: "pk_site_07c1e".to_string(), // TODO: Set from your business logic
    })
}
