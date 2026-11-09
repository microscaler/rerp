// Implementation stub for handler 'list_ad_creatives'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::list_ad_creatives::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_acquisition_gen::handlers::types::AdCreative;

#[handler(ListAdCreativesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let kind = req.inner.kind;// let account_id = req.inner.account_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-10-30T16:00:00Z","duration_seconds":3,"headline":"48 hours before everyone else","height":628,"id":"30f4c733-b7f5-51d2-b900-0be84807ebb2","kind":"IMAGE","landing_url":"https://acme.example/early-access","name":"BF hero","platforms":[{},{},{}],"review":{"GOOGLE_ADS":"APPROVED","LINKEDIN":"APPROVED","META":"APPROVED"},"width":1200})).unwrap_or_default(), serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-10-31T16:00:00Z","duration_seconds":3,"headline":"Early access · 21 Nov","height":1080,"id":"8767be08-2adf-5232-a68d-eefde450d501","kind":"IMAGE","landing_url":"https://acme.example/early-access","name":"BF square","platforms":[{},{}],"review":{"LINKEDIN":"PENDING","META":"APPROVED"},"width":1080})).unwrap_or_default(), serde_json::from_value::<AdCreative>(serde_json::json!({"body":"","brand_kit_applied":true,"call_to_action":"LEARN_MORE","created_at":"2026-11-01T16:00:00Z","duration_seconds":15,"headline":"Pricing that holds up","height":1080,"id":"3998574b-7118-511a-bd3b-fad92e7cddfd","kind":"VIDEO","landing_url":"https://acme.example/webinar","name":"Webinar","platforms":[{}],"review":{"LINKEDIN":"APPROVED"},"width":1920})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
