// Implementation stub for handler 'list_postal_addresses'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_postal_addresses::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::PostalAddress;

#[handler(ListPostalAddressesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Co, 12 Example Street, 10115 Berlin, Germany","id":"9c7a38b3-2c83-5eb5-9d4c-7cf723835c99","is_default":true,"name":"Berlin HQ","used_by":["Newsletter","Customers","Webinars"]})).unwrap_or_default(), serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Co (Pty) Ltd, 4 Sample Road, Cape Town 8001, South Africa","id":"0f860ccb-c518-552f-8ca2-767c52688164","is_default":false,"name":"Cape Town office","used_by":["Partners"]})).unwrap_or_default(), serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Events, PO Box 123, 10115 Berlin, Germany","id":"c699ea0a-3702-54fd-9422-9444801a3fa0","is_default":false,"name":"Events","used_by":[]})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
