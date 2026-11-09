// User-owned controller for handler 'get_attribution_overview'.

use crate::handlers::get_attribution_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAttributionOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "funnel": [
    //     {
    //       "count": 84200,
    //       "label": "Visitors"
    //     },
    //     {
    //       "count": 6114,
    //       "label": "Identified contacts"
    //     },
    //     {
    //       "count": 3529,
    //       "label": "Leads"
    //     }
    //   ],
    //   "rows": [
    //     {
    //       "contacts": 1104,
    //       "converted": 9,
    //       "key": "google / cpc",
    //       "leads": 612,
    //       "revenue": "38400.00"
    //     },
    //     {
    //       "contacts": 902,
    //       "converted": 6,
    //       "key": "linkedin / social",
    //       "leads": 418,
    //       "revenue": "41000.00"
    //     },
    //     {
    //       "contacts": 0,
    //       "converted": 7,
    //       "key": "newsletter / email",
    //       "leads": 301,
    //       "revenue": "12140.00"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "funnel": [
    {
      "count": 84200,
      "label": "Visitors"
    },
    {
      "count": 6114,
      "label": "Identified contacts"
    },
    {
      "count": 3529,
      "label": "Leads"
    }
  ],
  "rows": [
    {
      "contacts": 1104,
      "converted": 9,
      "key": "google / cpc",
      "leads": 612,
      "revenue": "38400.00"
    },
    {
      "contacts": 902,
      "converted": 6,
      "key": "linkedin / social",
      "leads": 418,
      "revenue": "41000.00"
    },
    {
      "contacts": 0,
      "converted": 7,
      "key": "newsletter / email",
      "leads": 301,
      "revenue": "12140.00"
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

    HttpJson::ok(Response {
        funnel: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"count":84200,"label":"Visitors"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":6114,"label":"Identified contacts"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":3529,"label":"Leads"}"#).unwrap_or(serde_json::Value::Null)]),rows: vec![serde_json::from_str::<serde_json::Value>(r#"{"contacts":1104,"converted":9,"key":"google / cpc","leads":612,"revenue":"38400.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contacts":902,"converted":6,"key":"linkedin / social","leads":418,"revenue":"41000.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contacts":0,"converted":7,"key":"newsletter / email","leads":301,"revenue":"12140.00"}"#).unwrap_or(serde_json::Value::Null)],
    })
}
