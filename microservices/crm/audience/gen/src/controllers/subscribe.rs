// User-owned controller for handler 'subscribe'.

use crate::handlers::subscribe::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConsentEvent;
#[allow(unused_imports)]
use crate::handlers::types::ConsentSource;
#[allow(unused_imports)]
use crate::handlers::types::SubscriptionStatus;

#[handler(SubscribeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "consent_at": "2026-10-31T16:00:00Z",
    //   "consent_ip": "84.22.0.x",
    //   "consent_source": "WEB_FORM",
    //   "consent_text": "I agree to receive product news by email.",
    //   "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "created_at": "2026-10-31T16:00:00Z",
    //   "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //   "history": [
    //     {
    //       "actor": "contact",
    //       "at": "2026-10-31T16:00:00Z",
    //       "source": "WEB_FORM",
    //       "status": "SUBSCRIBED"
    //     }
    //   ],
    //   "id": "cc5a9574-3a9f-54e6-be27-251e141be48a",
    //   "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //   "list_name": "Newsletter",
    //   "status": "SUBSCRIBED",
    //   "unsubscribe_reason": "Unsubscribe reason",
    //   "unsubscribed_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "consent_at": "2026-10-31T16:00:00Z",
  "consent_ip": "84.22.0.x",
  "consent_source": "WEB_FORM",
  "consent_text": "I agree to receive product news by email.",
  "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "created_at": "2026-10-31T16:00:00Z",
  "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
  "history": [
    {
      "actor": "contact",
      "at": "2026-10-31T16:00:00Z",
      "source": "WEB_FORM",
      "status": "SUBSCRIBED"
    }
  ],
  "id": "cc5a9574-3a9f-54e6-be27-251e141be48a",
  "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
  "list_name": "Newsletter",
  "status": "SUBSCRIBED",
  "unsubscribe_reason": "Unsubscribe reason",
  "unsubscribed_at": "2026-11-05T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        consent_at: Some("2026-10-31T16:00:00Z".to_string()),consent_ip: Some("84.22.0.x".to_string()),consent_source: Some(serde_json::from_value::<ConsentSource>(serde_json::Value::String("WEB_FORM".to_string())).unwrap_or_default()),consent_text: Some("I agree to receive product news by email.".to_string()),contact_id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(),created_at: "2026-10-31T16:00:00Z".to_string(),form_id: Some("454cdf11-2c7b-505e-8864-df16d18ca8cc".to_string()),history: Some(vec![serde_json::from_value::<ConsentEvent>(serde_json::json!({"actor":"contact","at":"2026-10-31T16:00:00Z","source":"WEB_FORM","status":"SUBSCRIBED"})).unwrap_or_default()]),id: "cc5a9574-3a9f-54e6-be27-251e141be48a".to_string(),list_id: "09f5b2f9-2adc-5edd-a6fd-c20675680671".to_string(),list_name: Some("Newsletter".to_string()),status: serde_json::from_value::<SubscriptionStatus>(serde_json::Value::String("SUBSCRIBED".to_string())).unwrap_or_default(),unsubscribe_reason: Some("Unsubscribe reason".to_string()),unsubscribed_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
