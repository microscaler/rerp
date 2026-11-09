// User-owned controller for handler 'update_form'.

use crate::handlers::update_form::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FormAction;
#[allow(unused_imports)]
use crate::handlers::types::FormFieldDef;
#[allow(unused_imports)]
use crate::handlers::types::FormStyle;

#[handler(UpdateFormController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "actions": [
    //     {
    //       "kind": "SUBSCRIBE_LIST",
    //       "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //       "tag": "Tag",
    //       "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //     },
    //     {
    //       "kind": "ADD_TAG",
    //       "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //       "tag": "newsletter",
    //       "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //     }
    //   ],
    //   "allowed_origins": [
    //     "https://acme.example"
    //   ],
    //   "capture_fields": {},
    //   "conversion_rate": 0.013,
    //   "created_at": "2026-04-19T16:00:00Z",
    //   "description": "",
    //   "double_opt_in": true,
    //   "fields": [
    //     {
    //       "key": "email",
    //       "kind": "EMAIL",
    //       "label": "Work email",
    //       "required": true
    //     },
    //     {
    //       "key": "first_name",
    //       "kind": "TEXT",
    //       "label": "First name",
    //       "required": false
    //     },
    //     {
    //       "key": "custom:team_size",
    //       "kind": "SELECT",
    //       "label": "Team size",
    //       "options": [
    //         "1-10",
    //         "11-50",
    //         "51+"
    //       ]
    //     }
    //   ],
    //   "id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //   "is_active": true,
    //   "name": "Website footer",
    //   "public_url": "https://crm.acme.example/p/website-footer",
    //   "style": {
    //     "layout": "INLINE",
    //     "submit_label": "Join",
    //     "success_message": "Thanks, check your inbox to confirm."
    //   },
    //   "submission_count": 612
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "actions": [
    {
      "kind": "SUBSCRIBE_LIST",
      "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
      "tag": "Tag",
      "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    },
    {
      "kind": "ADD_TAG",
      "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
      "tag": "newsletter",
      "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    }
  ],
  "allowed_origins": [
    "https://acme.example"
  ],
  "capture_fields": {},
  "conversion_rate": 0.013,
  "created_at": "2026-04-19T16:00:00Z",
  "description": "",
  "double_opt_in": true,
  "fields": [
    {
      "key": "email",
      "kind": "EMAIL",
      "label": "Work email",
      "required": true
    },
    {
      "key": "first_name",
      "kind": "TEXT",
      "label": "First name",
      "required": false
    },
    {
      "key": "custom:team_size",
      "kind": "SELECT",
      "label": "Team size",
      "options": [
        "1-10",
        "11-50",
        "51+"
      ]
    }
  ],
  "id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
  "is_active": true,
  "name": "Website footer",
  "public_url": "https://crm.acme.example/p/website-footer",
  "style": {
    "layout": "INLINE",
    "submit_label": "Join",
    "success_message": "Thanks, check your inbox to confirm."
  },
  "submission_count": 612
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        actions: Some(vec![serde_json::from_value::<FormAction>(serde_json::json!({"kind":"SUBSCRIBE_LIST","list_id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","tag":"Tag","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"})).unwrap_or_default(), serde_json::from_value::<FormAction>(serde_json::json!({"kind":"ADD_TAG","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"newsletter","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"})).unwrap_or_default()]),allow_blank_overwrite: Some(true),allowed_origins: Some(vec!["https://acme.example".to_string()]),api_endpoint: Some("example".to_string()),block_free_mail: Some(true),blocked_domains: Some(vec![]),capture_fields: serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),consent_required: Some(true),consent_text: Some("example".to_string()),conversion_rate: Some(0.013),created_at: Some("2026-04-19T16:00:00Z".to_string()),custom_html: Some("example".to_string()),description: Some("".to_string()),double_opt_in: Some(true),embed_code: Some("example".to_string()),fields: Some(vec![serde_json::from_value::<FormFieldDef>(serde_json::json!({"key":"email","kind":"EMAIL","label":"Work email","required":true})).unwrap_or_default(), serde_json::from_value::<FormFieldDef>(serde_json::json!({"key":"first_name","kind":"TEXT","label":"First name","required":false})).unwrap_or_default(), serde_json::from_value::<FormFieldDef>(serde_json::json!({"key":"custom:team_size","kind":"SELECT","label":"Team size","options":["1-10","11-50","51+"]})).unwrap_or_default()]),id: "454cdf11-2c7b-505e-8864-df16d18ca8cc".to_string(),is_active: Some(true),name: "Website footer".to_string(),on_submit: Some("example".to_string()),public_url: Some("https://crm.acme.example/p/website-footer".to_string()),redirect_url: Some("example".to_string()),style: Some(serde_json::from_value::<FormStyle>(serde_json::json!({"layout":"INLINE","submit_label":"Join","success_message":"Thanks, check your inbox to confirm."})).unwrap_or_default()),submission_count: Some(612),team_id: Some("example".to_string()),user_id: Some("example".to_string()),utm_campaign_id: Some("example".to_string()),utm_medium_id: Some("example".to_string()),utm_source_id: Some("example".to_string()),workflow_ids: Some(vec![]),
    })
}
