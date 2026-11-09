// Implementation stub for handler 'list_deal_roles'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_deal_roles::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::DealRole;

#[handler(ListDealRolesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DealRole>(serde_json::json!({"description":"Default role for the deal's first contact","id":"c0fd9836-bb8e-5080-b832-50c4843b3e9f","title":"Primary contact","used_on":38})).unwrap_or_default(), serde_json::from_value::<DealRole>(serde_json::json!({"description":"Signs off budget","id":"13e07af8-0ff7-55cb-92ef-95d54cd5b873","title":"Decision maker","used_on":14})).unwrap_or_default(), serde_json::from_value::<DealRole>(serde_json::json!({"description":"Internal advocate","id":"77c6efec-76dd-550c-bf81-5771cb2b67ac","title":"Champion","used_on":11})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
