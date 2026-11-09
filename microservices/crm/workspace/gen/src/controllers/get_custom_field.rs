// User-owned controller for handler 'get_custom_field'.

use crate::handlers::get_custom_field::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FieldType;

#[handler(GetCustomFieldController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entity": "contact",
    //   "field_type": "STRING",
    //   "group": "Profile",
    //   "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
    //   "label": "Plan",
    //   "name": "Northwind Traders",
    //   "options": {},
    //   "required": false,
    //   "searchable": false,
    //   "sequence": 1,
    //   "show_on_card": false,
    //   "visible": false
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "entity": "contact",
  "field_type": "STRING",
  "group": "Profile",
  "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
  "label": "Plan",
  "name": "Northwind Traders",
  "options": {},
  "required": false,
  "searchable": false,
  "sequence": 1,
  "show_on_card": false,
  "visible": false
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        default_value: Some("example".to_string()),
        entity: "contact".to_string(),
        field_type: serde_json::from_value::<FieldType>(serde_json::Value::String(
            "STRING".to_string(),
        ))
        .unwrap_or_default(),
        group: Some("Profile".to_string()),
        id: "9851eff3-bcf4-59e1-9c43-82ba49bdc08c".to_string(),
        label: "Plan".to_string(),
        name: "Northwind Traders".to_string(),
        options: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        personalization_tag: Some("example".to_string()),
        required: Some(false),
        searchable: Some(false),
        sequence: Some(1),
        show_on_card: Some(false),
        visible: false,
    })
}
