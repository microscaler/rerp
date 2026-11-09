// User-owned controller for handler 'list_conversion_definitions'.

use crate::handlers::list_conversion_definitions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConversionDefinition;

#[handler(ListConversionDefinitionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "count_30d": 2140,
    //       "created_at": "2026-06-08T16:00:00Z",
    //       "currency": "EUR",
    //       "id": "30eefab7-ff42-50cf-85bd-3a974b1ce548",
    //       "last_at": "2026-11-05T15:51:00Z",
    //       "name": "Trial signup",
    //       "once_per_contact": true,
    //       "revenue_30d": "0.00",
    //       "total": 6114,
    //       "trigger": {
    //         "kind": "FORM_SUBMITTED",
    //         "ref": "Free trial"
    //       },
    //       "value": "0.00",
    //       "value_mode": "NONE"
    //     },
    //     {
    //       "count_30d": 1312,
    //       "created_at": "2026-06-08T16:00:00Z",
    //       "currency": "EUR",
    //       "id": "473f4a5c-dcac-59e5-b889-2eb270d9db5d",
    //       "last_at": "2026-11-05T15:58:00Z",
    //       "name": "Activated",
    //       "once_per_contact": true,
    //       "revenue_30d": "52480.00",
    //       "total": 3988,
    //       "trigger": {
    //         "kind": "FORM_SUBMITTED",
    //         "ref": "activated"
    //       },
    //       "value": "40.00",
    //       "value_mode": "FIXED"
    //     },
    //     {
    //       "count_30d": 58,
    //       "created_at": "2026-06-08T16:00:00Z",
    //       "currency": "EUR",
    //       "id": "62363f71-61de-5e1b-ab25-896ce248a502",
    //       "last_at": "2026-11-05T13:00:00Z",
    //       "name": "Booked a demo",
    //       "once_per_contact": true,
    //       "revenue_30d": "14500.00",
    //       "total": 412,
    //       "trigger": {
    //         "kind": "FORM_SUBMITTED",
    //         "ref": "Demo request"
    //       },
    //       "value": "250.00",
    //       "value_mode": "FIXED"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
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
    },
    {
      "count_30d": 1312,
      "created_at": "2026-06-08T16:00:00Z",
      "currency": "EUR",
      "id": "473f4a5c-dcac-59e5-b889-2eb270d9db5d",
      "last_at": "2026-11-05T15:58:00Z",
      "name": "Activated",
      "once_per_contact": true,
      "revenue_30d": "52480.00",
      "total": 3988,
      "trigger": {
        "kind": "FORM_SUBMITTED",
        "ref": "activated"
      },
      "value": "40.00",
      "value_mode": "FIXED"
    },
    {
      "count_30d": 58,
      "created_at": "2026-06-08T16:00:00Z",
      "currency": "EUR",
      "id": "62363f71-61de-5e1b-ab25-896ce248a502",
      "last_at": "2026-11-05T13:00:00Z",
      "name": "Booked a demo",
      "once_per_contact": true,
      "revenue_30d": "14500.00",
      "total": 412,
      "trigger": {
        "kind": "FORM_SUBMITTED",
        "ref": "Demo request"
      },
      "value": "250.00",
      "value_mode": "FIXED"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":2140,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"30eefab7-ff42-50cf-85bd-3a974b1ce548","last_at":"2026-11-05T15:51:00Z","name":"Trial signup","once_per_contact":true,"revenue_30d":"0.00","total":6114,"trigger":{"kind":"FORM_SUBMITTED","ref":"Free trial"},"value":"0.00","value_mode":"NONE"})).unwrap_or_default(), serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":1312,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"473f4a5c-dcac-59e5-b889-2eb270d9db5d","last_at":"2026-11-05T15:58:00Z","name":"Activated","once_per_contact":true,"revenue_30d":"52480.00","total":3988,"trigger":{"kind":"FORM_SUBMITTED","ref":"activated"},"value":"40.00","value_mode":"FIXED"})).unwrap_or_default(), serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":58,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"62363f71-61de-5e1b-ab25-896ce248a502","last_at":"2026-11-05T13:00:00Z","name":"Booked a demo","once_per_contact":true,"revenue_30d":"14500.00","total":412,"trigger":{"kind":"FORM_SUBMITTED","ref":"Demo request"},"value":"250.00","value_mode":"FIXED"})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
