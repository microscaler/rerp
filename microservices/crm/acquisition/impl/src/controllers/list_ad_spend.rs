// Implementation stub for handler 'list_ad_spend'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::list_ad_spend::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_acquisition_gen::handlers::types::AdSpendRow;

#[handler(ListAdSpendController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let account_id = req.inner.account_id;// let from = req.inner.from;// let to = req.inner.to;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","campaign":"BF early access · warm-up","clicks":27,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":8640,"platform":"META","spend":"180.00"})).unwrap_or_default(), serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","campaign":"Brand search","clicks":8,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":2592,"platform":"GOOGLE_ADS","spend":"54.00"})).unwrap_or_default(), serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","campaign":"Webinar invite · warm-up","clicks":9,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":2880,"platform":"LINKEDIN","spend":"60.00"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 21, // TODO: Set from your business logic
    })
}
