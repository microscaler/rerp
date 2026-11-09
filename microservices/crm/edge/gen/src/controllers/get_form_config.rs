// User-owned controller for handler 'get_form_config'.

use crate::handlers::get_form_config::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FormAction;
#[allow(unused_imports)]
use crate::handlers::types::FormFieldDef;
#[allow(unused_imports)]
use crate::handlers::types::FormStyle;

#[handler(GetFormConfigController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "actions": [
    //     {
    //       "kind": "SUBSCRIBE_LIST"
    //     }
    //   ],
    //   "allow_blank_overwrite": false,
    //   "allowed_origins": [
    //     "https://northwindtraders.example/pricing"
    //   ],
    //   "api_endpoint": "Api endpoint",
    //   "block_free_mail": false,
    //   "blocked_domains": [
    //     "Blocked domains"
    //   ],
    //   "capture_fields": {},
    //   "consent_required": false,
    //   "consent_text": "Consent text",
    //   "conversion_rate": 12.5,
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "custom_html": "Custom html",
    //   "description": "Example",
    //   "double_opt_in": false,
    //   "embed_code": "Embed code",
    //   "fields": [
    //     {
    //       "key": "snippet.signature",
    //       "kind": "TEXT",
    //       "label": "Primary"
    //     }
    //   ],
    //   "id": "9fa51332-6221-502c-a499-80ae6ddde550",
    //   "is_active": true,
    //   "name": "Northwind Traders",
    //   "on_submit": "THANK_YOU",
    //   "public_url": "https://northwindtraders.example/pricing",
    //   "redirect_url": "Redirect url",
    //   "style": {
    //     "layout": "INLINE",
    //     "submit_label": "Submit label",
    //     "success_message": "Success message",
    //     "theme": {},
    //     "trigger": {}
    //   },
    //   "submission_count": 3,
    //   "team_id": "58c109b8-638d-5291-b28c-af97be8e3ee1",
    //   "user_id": "alex",
    //   "utm_campaign_id": "a3603147-303a-5f7a-93aa-73df90f2a0ee",
    //   "utm_medium_id": "d01dfabd-7761-5c37-933a-1da620c32b92",
    //   "utm_source_id": "67ca3504-b03e-5297-a195-b111bb9a54b2",
    //   "workflow_ids": [
    //     "93fc52f4-00e0-548c-98fc-b9faa5056849"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "actions": [
    {
      "kind": "SUBSCRIBE_LIST"
    }
  ],
  "allow_blank_overwrite": false,
  "allowed_origins": [
    "https://northwindtraders.example/pricing"
  ],
  "api_endpoint": "Api endpoint",
  "block_free_mail": false,
  "blocked_domains": [
    "Blocked domains"
  ],
  "capture_fields": {},
  "consent_required": false,
  "consent_text": "Consent text",
  "conversion_rate": 12.5,
  "created_at": "2026-11-05T16:00:00Z",
  "custom_html": "Custom html",
  "description": "Example",
  "double_opt_in": false,
  "embed_code": "Embed code",
  "fields": [
    {
      "key": "snippet.signature",
      "kind": "TEXT",
      "label": "Primary"
    }
  ],
  "id": "9fa51332-6221-502c-a499-80ae6ddde550",
  "is_active": true,
  "name": "Northwind Traders",
  "on_submit": "THANK_YOU",
  "public_url": "https://northwindtraders.example/pricing",
  "redirect_url": "Redirect url",
  "style": {
    "layout": "INLINE",
    "submit_label": "Submit label",
    "success_message": "Success message",
    "theme": {},
    "trigger": {}
  },
  "submission_count": 3,
  "team_id": "58c109b8-638d-5291-b28c-af97be8e3ee1",
  "user_id": "alex",
  "utm_campaign_id": "a3603147-303a-5f7a-93aa-73df90f2a0ee",
  "utm_medium_id": "d01dfabd-7761-5c37-933a-1da620c32b92",
  "utm_source_id": "67ca3504-b03e-5297-a195-b111bb9a54b2",
  "workflow_ids": [
    "93fc52f4-00e0-548c-98fc-b9faa5056849"
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        actions: Some(vec![serde_json::from_value::<FormAction>(serde_json::json!({"kind":"SUBSCRIBE_LIST"})).unwrap_or_default()]),allow_blank_overwrite: Some(false),allowed_origins: Some(vec!["https://northwindtraders.example/pricing".to_string()]),api_endpoint: Some("Api endpoint".to_string()),block_free_mail: Some(false),blocked_domains: Some(vec!["Blocked domains".to_string()]),capture_fields: serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),consent_required: Some(false),consent_text: Some("Consent text".to_string()),conversion_rate: Some(12.5),created_at: Some("2026-11-05T16:00:00Z".to_string()),custom_html: Some("Custom html".to_string()),description: Some("Example".to_string()),double_opt_in: Some(false),embed_code: Some("Embed code".to_string()),fields: Some(vec![serde_json::from_value::<FormFieldDef>(serde_json::json!({"key":"snippet.signature","kind":"TEXT","label":"Primary"})).unwrap_or_default()]),id: "9fa51332-6221-502c-a499-80ae6ddde550".to_string(),is_active: Some(true),name: "Northwind Traders".to_string(),on_submit: Some("THANK_YOU".to_string()),public_url: Some("https://northwindtraders.example/pricing".to_string()),redirect_url: Some("Redirect url".to_string()),style: Some(serde_json::from_value::<FormStyle>(serde_json::json!({"layout":"INLINE","submit_label":"Submit label","success_message":"Success message","theme":{},"trigger":{}})).unwrap_or_default()),submission_count: Some(3),team_id: Some("58c109b8-638d-5291-b28c-af97be8e3ee1".to_string()),user_id: Some("alex".to_string()),utm_campaign_id: Some("a3603147-303a-5f7a-93aa-73df90f2a0ee".to_string()),utm_medium_id: Some("d01dfabd-7761-5c37-933a-1da620c32b92".to_string()),utm_source_id: Some("67ca3504-b03e-5297-a195-b111bb9a54b2".to_string()),workflow_ids: Some(vec!["93fc52f4-00e0-548c-98fc-b9faa5056849".to_string()]),
    })
}
