// User-owned controller for handler 'create_custom_field'.

use crate::handlers::create_custom_field::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FieldType;

#[handler(CreateCustomFieldController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entity": "contact",
    //   "group": "Profile",
    //   "hint": "",
    //   "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
    //   "key": "plan",
    //   "label": "Plan",
    //   "merge_field": "custom.plan",
    //   "options": {},
    //   "required": false,
    //   "searchable": false,
    //   "sequence": 1,
    //   "show_on_card": false,
    //   "type": "SELECT",
    //   "visible": false
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "entity": "contact",
  "group": "Profile",
  "hint": "",
  "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
  "key": "plan",
  "label": "Plan",
  "merge_field": "custom.plan",
  "options": {},
  "required": false,
  "searchable": false,
  "sequence": 1,
  "show_on_card": false,
  "type": "SELECT",
  "visible": false
}"#,
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
        group: Some("Profile".to_string()),
        hint: Some("".to_string()),
        id: "9851eff3-bcf4-59e1-9c43-82ba49bdc08c".to_string(),
        key: "plan".to_string(),
        label: "Plan".to_string(),
        merge_field: Some("custom.plan".to_string()),
        options: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        personalization_tag: Some("example".to_string()),
        required: Some(false),
        required_at_stage: Some("example".to_string()),
        searchable: Some(false),
        sequence: Some(1),
        show_on_card: Some(false),
        r#type: serde_json::from_value::<FieldType>(serde_json::Value::String(
            "SELECT".to_string(),
        ))
        .unwrap_or_default(),
        visible: false,
    })
}
