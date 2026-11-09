// User-owned controller for handler 'list_ad_adapters'.

use crate::handlers::list_ad_adapters::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAdapter;

#[handler(ListAdAdaptersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "capabilities": [
    //         "AUDIENCES",
    //         "LOOKALIKES",
    //         "CONVERSIONS"
    //       ],
    //       "identity": [
    //         "hashed email",
    //         "phone",
    //         "MAID"
    //       ],
    //       "kind": "WALLED_GARDEN_API",
    //       "minimums": {
    //         "audience_matched": 100
    //       },
    //       "notes": "Audience ≥ ~100 matched",
    //       "platform": "META",
    //       "status": "SHIPPED"
    //     },
    //     {
    //       "capabilities": [
    //         "AUDIENCES",
    //         "CONVERSIONS",
    //         "SPEND"
    //       ],
    //       "identity": [
    //         "hashed email (Customer Match)",
    //         "gclid"
    //       ],
    //       "kind": "WALLED_GARDEN_API",
    //       "minimums": {
    //         "audience_matched": 1000
    //       },
    //       "notes": "Customer Match ≥ 1,000 · developer token",
    //       "platform": "GOOGLE_ADS",
    //       "status": "SHIPPED"
    //     },
    //     {
    //       "capabilities": [
    //         "AUDIENCES",
    //         "CONVERSIONS",
    //         "SPEND"
    //       ],
    //       "identity": [
    //         "hashed work email",
    //         "company lists"
    //       ],
    //       "kind": "WALLED_GARDEN_API",
    //       "minimums": {},
    //       "notes": "No minimum · best B2B match",
    //       "platform": "LINKEDIN",
    //       "status": "SHIPPED"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "capabilities": [
        "AUDIENCES",
        "LOOKALIKES",
        "CONVERSIONS"
      ],
      "identity": [
        "hashed email",
        "phone",
        "MAID"
      ],
      "kind": "WALLED_GARDEN_API",
      "minimums": {
        "audience_matched": 100
      },
      "notes": "Audience ≥ ~100 matched",
      "platform": "META",
      "status": "SHIPPED"
    },
    {
      "capabilities": [
        "AUDIENCES",
        "CONVERSIONS",
        "SPEND"
      ],
      "identity": [
        "hashed email (Customer Match)",
        "gclid"
      ],
      "kind": "WALLED_GARDEN_API",
      "minimums": {
        "audience_matched": 1000
      },
      "notes": "Customer Match ≥ 1,000 · developer token",
      "platform": "GOOGLE_ADS",
      "status": "SHIPPED"
    },
    {
      "capabilities": [
        "AUDIENCES",
        "CONVERSIONS",
        "SPEND"
      ],
      "identity": [
        "hashed work email",
        "company lists"
      ],
      "kind": "WALLED_GARDEN_API",
      "minimums": {},
      "notes": "No minimum · best B2B match",
      "platform": "LINKEDIN",
      "status": "SHIPPED"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<AdAdapter>(serde_json::json!({"capabilities":["AUDIENCES","LOOKALIKES","CONVERSIONS"],"identity":["hashed email","phone","MAID"],"kind":"WALLED_GARDEN_API","minimums":{"audience_matched":100},"notes":"Audience ≥ ~100 matched","platform":"META","status":"SHIPPED"})).unwrap_or_default(), serde_json::from_value::<AdAdapter>(serde_json::json!({"capabilities":["AUDIENCES","CONVERSIONS","SPEND"],"identity":["hashed email (Customer Match)","gclid"],"kind":"WALLED_GARDEN_API","minimums":{"audience_matched":1000},"notes":"Customer Match ≥ 1,000 · developer token","platform":"GOOGLE_ADS","status":"SHIPPED"})).unwrap_or_default(), serde_json::from_value::<AdAdapter>(serde_json::json!({"capabilities":["AUDIENCES","CONVERSIONS","SPEND"],"identity":["hashed work email","company lists"],"kind":"WALLED_GARDEN_API","minimums":{},"notes":"No minimum · best B2B match","platform":"LINKEDIN","status":"SHIPPED"})).unwrap_or_default()]))
}
