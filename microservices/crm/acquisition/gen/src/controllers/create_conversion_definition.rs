// User-owned controller for handler 'create_conversion_definition'.

use crate::handlers::create_conversion_definition::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConversionTrigger;

#[handler(CreateConversionDefinitionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count_30d": 2140,
    //   "created_at": "2026-06-08T16:00:00Z",
    //   "currency": "EUR",
    //   "id": "30eefab7-ff42-50cf-85bd-3a974b1ce548",
    //   "last_at": "2026-11-05T15:51:00Z",
    //   "name": "Trial signup",
    //   "once_per_contact": true,
    //   "revenue_30d": "0.00",
    //   "total": 6114,
    //   "trigger": {
    //     "kind": "FORM_SUBMITTED",
    //     "ref": "Free trial"
    //   },
    //   "value": "0.00",
    //   "value_mode": "NONE"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "count_30d": 2140,
  "created_at": "2026-06-08T16:00:00Z",
  "currency": "EUR",
  "id": "30eefab7-ff42-50cf-85bd-3a974b1ce548",
  "last_at": "2026-11-05T15:51:00Z",
  "name": "Trial signup",
  "once_per_contact": true,
  "revenue_30d": "0.00",
  "total": 6114,
  "trigger": {
    "kind": "FORM_SUBMITTED",
    "ref": "Free trial"
  },
  "value": "0.00",
  "value_mode": "NONE"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        count_30d: Some(2140),
        created_at: Some("2026-06-08T16:00:00Z".to_string()),
        currency: Some("EUR".to_string()),
        id: "30eefab7-ff42-50cf-85bd-3a974b1ce548".to_string(),
        last_at: Some("2026-11-05T15:51:00Z".to_string()),
        name: "Trial signup".to_string(),
        once_per_contact: Some(true),
        revenue_30d: Some("0.00".to_string()),
        total: Some(6114),
        trigger: serde_json::from_value::<ConversionTrigger>(
            serde_json::json!({"kind":"FORM_SUBMITTED","ref":"Free trial"}),
        )
        .unwrap_or_default(),
        value: Some("0.00".to_string()),
        value_mode: Some("NONE".to_string()),
    })
}
