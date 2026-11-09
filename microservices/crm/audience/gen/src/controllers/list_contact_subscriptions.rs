// User-owned controller for handler 'list_contact_subscriptions'.

use crate::handlers::list_contact_subscriptions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Subscription;

#[handler(ListContactSubscriptionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "consent_at": "2026-10-31T16:00:00Z",
    //       "consent_ip": "84.22.0.x",
    //       "consent_source": "WEB_FORM",
    //       "consent_text": "I agree to receive product news by email.",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-10-31T16:00:00Z",
    //       "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //       "history": [
    //         {
    //           "actor": "contact",
    //           "at": "2026-10-31T16:00:00Z",
    //           "source": "WEB_FORM",
    //           "status": "SUBSCRIBED"
    //         }
    //       ],
    //       "id": "cc5a9574-3a9f-54e6-be27-251e141be48a",
    //       "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //       "list_name": "Newsletter",
    //       "status": "SUBSCRIBED",
    //       "unsubscribe_reason": "Unsubscribe reason",
    //       "unsubscribed_at": "2026-11-05T16:00:00Z"
    //     },
    //     {
    //       "consent_at": "2026-10-31T16:00:00Z",
    //       "consent_ip": "84.22.0.x",
    //       "consent_source": "WEB_FORM",
    //       "consent_text": "I agree to receive product news by email.",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-10-31T16:00:00Z",
    //       "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //       "history": [
    //         {
    //           "actor": "contact",
    //           "at": "2026-10-31T16:00:00Z",
    //           "source": "WEB_FORM",
    //           "status": "SUBSCRIBED"
    //         }
    //       ],
    //       "id": "44170062-95a2-508d-83f1-6143ddbd526a",
    //       "list_id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
    //       "list_name": "Customers",
    //       "status": "SUBSCRIBED",
    //       "unsubscribe_reason": "Unsubscribe reason",
    //       "unsubscribed_at": "2026-11-05T16:00:00Z"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
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
    },
    {
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
      "id": "44170062-95a2-508d-83f1-6143ddbd526a",
      "list_id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
      "list_name": "Customers",
      "status": "SUBSCRIBED",
      "unsubscribe_reason": "Unsubscribe reason",
      "unsubscribed_at": "2026-11-05T16:00:00Z"
    }
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<Subscription>(serde_json::json!({"consent_at":"2026-10-31T16:00:00Z","consent_ip":"84.22.0.x","consent_source":"WEB_FORM","consent_text":"I agree to receive product news by email.","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-31T16:00:00Z","form_id":"454cdf11-2c7b-505e-8864-df16d18ca8cc","history":[{"actor":"contact","at":"2026-10-31T16:00:00Z","source":"WEB_FORM","status":"SUBSCRIBED"}],"id":"cc5a9574-3a9f-54e6-be27-251e141be48a","list_id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","list_name":"Newsletter","status":"SUBSCRIBED","unsubscribe_reason":"Unsubscribe reason","unsubscribed_at":"2026-11-05T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Subscription>(serde_json::json!({"consent_at":"2026-10-31T16:00:00Z","consent_ip":"84.22.0.x","consent_source":"WEB_FORM","consent_text":"I agree to receive product news by email.","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-31T16:00:00Z","form_id":"454cdf11-2c7b-505e-8864-df16d18ca8cc","history":[{"actor":"contact","at":"2026-10-31T16:00:00Z","source":"WEB_FORM","status":"SUBSCRIBED"}],"id":"44170062-95a2-508d-83f1-6143ddbd526a","list_id":"f84269dd-7dba-5a82-ad63-986af03f39c1","list_name":"Customers","status":"SUBSCRIBED","unsubscribe_reason":"Unsubscribe reason","unsubscribed_at":"2026-11-05T16:00:00Z"})).unwrap_or_default()]))
}
