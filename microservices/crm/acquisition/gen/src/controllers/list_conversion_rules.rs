// User-owned controller for handler 'list_conversion_rules'.

use crate::handlers::list_conversion_rules::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdConversionRule;

#[handler(ListConversionRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
    //       "event": "FORM_SUBMITTED"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
      "event": "FORM_SUBMITTED"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<AdConversionRule>(serde_json::json!({"account_id":"bafc0e6d-6308-524f-9487-8d8424615bb7","event":"FORM_SUBMITTED"})).unwrap_or_default()]))
}
