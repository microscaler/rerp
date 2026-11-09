// User-owned controller for handler 'record_ad_conversion'.

use crate::handlers::record_ad_conversion::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdConversion;

#[handler(RecordAdConversionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //       "status": "QUEUED"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
      "status": "QUEUED"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<AdConversion>(
        serde_json::json!({"id":"e67555dd-33f4-5fee-9da3-a019895219b1","status":"QUEUED"}),
    )
    .unwrap_or_default()]))
}
