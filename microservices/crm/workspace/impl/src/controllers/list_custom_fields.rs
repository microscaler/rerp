// Implementation stub for handler 'list_custom_fields'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_custom_fields::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::CustomField;

#[handler(ListCustomFieldsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let entity = req.inner.entity;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"9851eff3-bcf4-59e1-9c43-82ba49bdc08c","key":"plan","label":"Plan","merge_field":"custom.plan","options":{},"required":false,"searchable":false,"sequence":1,"show_on_card":false,"type":"SELECT","visible":false})).unwrap_or_default(), serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"6cb8ed49-8877-5cc2-bc25-eafe10a4a357","key":"renewal_date","label":"Renewal date","merge_field":"custom.renewal_date","required":false,"searchable":false,"sequence":2,"show_on_card":false,"type":"DATE","visible":false})).unwrap_or_default(), serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"8b2236e2-bcc8-589a-baaf-fa871172e5ee","key":"seats","label":"Seats","merge_field":"custom.seats","required":false,"searchable":false,"sequence":3,"show_on_card":false,"type":"STRING","visible":false})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 20, // TODO: Set from your business logic
    })
}
