// User-owned controller for handler 'update_edi_mapping'.

use crate::handlers::update_edi_mapping::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateEdiMappingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        default_values: Some(Default::default()),
        description: "example".to_string(),
        document_type: "example".to_string(),
        field_mappings: Default::default(),
        format_id: "example".to_string(),
        id: "example".to_string(),
        is_active: true,
        mapping_name: "example".to_string(),
        transformation_rules: Some(Default::default()),
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
        validation_rules: Some(Default::default()),
    })
}
