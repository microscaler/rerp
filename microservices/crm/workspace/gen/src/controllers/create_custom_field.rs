// User-owned controller for handler 'create_custom_field'.

use crate::handlers::create_custom_field::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FieldType;

#[handler(CreateCustomFieldController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        default_value: Some("example".to_string()),
        entity: "example".to_string(),
        field_type: Default::default(),
        group: Some("example".to_string()),
        id: "example".to_string(),
        label: "example".to_string(),
        name: "example".to_string(),
        options: Some(Default::default()),
        personalization_tag: Some("example".to_string()),
        required: Some(true),
        searchable: Some(true),
        sequence: Some(42),
        show_on_card: Some(true),
        visible: true,
    })
}
