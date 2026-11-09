// User-owned controller for handler 'create_conversion_rule'.

use crate::handlers::create_conversion_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateConversionRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //   "consent_required": true,
    //   "currency": "EUR",
    //   "dedupe_key": "DEAL_ID",
    //   "enabled": true,
    //   "event": "FORM_SUBMITTED",
    //   "event_filter": {},
    //   "external_event_name": "Lead",
    //   "failed_30d": 3,
    //   "fixed_value": "40.00",
    //   "id": "6d83f7aa-792a-54cd-a144-a8810febc825",
    //   "name": "Form submitted · any",
    //   "require_click": false,
    //   "sent_30d": 612,
    //   "value_from": "FIXED",
    //   "window_days": 30
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
  "consent_required": true,
  "currency": "EUR",
  "dedupe_key": "DEAL_ID",
  "enabled": true,
  "event": "FORM_SUBMITTED",
  "event_filter": {},
  "external_event_name": "Lead",
  "failed_30d": 3,
  "fixed_value": "40.00",
  "id": "6d83f7aa-792a-54cd-a144-a8810febc825",
  "name": "Form submitted · any",
  "require_click": false,
  "sent_30d": 612,
  "value_from": "FIXED",
  "window_days": 30
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        account_id: "ded95315-6c77-51a0-b8ea-66b76fada21a".to_string(),
        consent_required: Some(true),
        currency: Some("EUR".to_string()),
        dedupe_key: Some("DEAL_ID".to_string()),
        enabled: Some(true),
        event: "FORM_SUBMITTED".to_string(),
        event_filter: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        external_event_name: Some("Lead".to_string()),
        failed_30d: Some(3),
        fixed_value: Some("40.00".to_string()),
        id: Some("6d83f7aa-792a-54cd-a144-a8810febc825".to_string()),
        name: Some("Form submitted · any".to_string()),
        require_click: Some(false),
        sent_30d: Some(612),
        value_from: Some("FIXED".to_string()),
        window_days: Some(30),
    })
}
