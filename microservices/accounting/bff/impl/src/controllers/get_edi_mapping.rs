// User-owned controller for handler 'get_edi_mapping'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::get_edi_mapping::{Request, Response};

#[handler(GetEdiMappingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        default_values: Some(Default::default()),
        description: Some("example".to_string()),
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
    }
}
