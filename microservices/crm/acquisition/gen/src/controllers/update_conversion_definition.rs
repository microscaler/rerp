// User-owned controller for handler 'update_conversion_definition'.

use crate::handlers::update_conversion_definition::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConversionTrigger;

#[handler(UpdateConversionDefinitionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count_30d": 3,
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "currency": "EUR",
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "last_at": "2026-11-05T16:00:00Z",
    //   "name": "Northwind Traders",
    //   "once_per_contact": false,
    //   "revenue_30d": "1250.00",
    //   "total": 2,
    //   "trigger": {
    //     "kind": "FORM_SUBMITTED",
    //     "ref": "Ref"
    //   },
    //   "value": "1250.00",
    //   "value_mode": "NONE"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "count_30d": 3,
  "created_at": "2026-11-05T16:00:00Z",
  "currency": "EUR",
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "last_at": "2026-11-05T16:00:00Z",
  "name": "Northwind Traders",
  "once_per_contact": false,
  "revenue_30d": "1250.00",
  "total": 2,
  "trigger": {
    "kind": "FORM_SUBMITTED",
    "ref": "Ref"
  },
  "value": "1250.00",
  "value_mode": "NONE"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        count_30d: Some(3),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        currency: Some("EUR".to_string()),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        last_at: Some("2026-11-05T16:00:00Z".to_string()),
        name: "Northwind Traders".to_string(),
        once_per_contact: Some(false),
        revenue_30d: Some("1250.00".to_string()),
        total: Some(2),
        trigger: serde_json::from_value::<ConversionTrigger>(
            serde_json::json!({"kind":"FORM_SUBMITTED","ref":"Ref"}),
        )
        .unwrap_or_default(),
        value: Some("1250.00".to_string()),
        value_mode: Some("NONE".to_string()),
    })
}
