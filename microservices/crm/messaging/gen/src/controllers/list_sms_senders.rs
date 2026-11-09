// User-owned controller for handler 'list_sms_senders'.

use crate::handlers::list_sms_senders::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::SmsSender;

#[handler(ListSmsSendersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "countries": [
    //         "DE",
    //         "UK",
    //         "FR"
    //       ],
    //       "id": "b58c0df7-9372-505d-87f2-2b050e076e26",
    //       "kind": "ALPHANUMERIC",
    //       "label": "ACME (alphanumeric)",
    //       "verified": true
    //     },
    //     {
    //       "countries": [
    //         "DE"
    //       ],
    //       "id": "af48362e-151e-57dc-bdbc-56d8053ddaab",
    //       "kind": "ALPHANUMERIC",
    //       "label": "+49 157 … (German long number)",
    //       "verified": true
    //     },
    //     {
    //       "countries": [
    //         "US",
    //         "CA"
    //       ],
    //       "id": "72e724be-5121-5d5d-ab50-1ba7296bf2dd",
    //       "kind": "TOLL_FREE",
    //       "label": "+1 (833) … (US toll-free)",
    //       "verified": true
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "countries": [
        "DE",
        "UK",
        "FR"
      ],
      "id": "b58c0df7-9372-505d-87f2-2b050e076e26",
      "kind": "ALPHANUMERIC",
      "label": "ACME (alphanumeric)",
      "verified": true
    },
    {
      "countries": [
        "DE"
      ],
      "id": "af48362e-151e-57dc-bdbc-56d8053ddaab",
      "kind": "ALPHANUMERIC",
      "label": "+49 157 … (German long number)",
      "verified": true
    },
    {
      "countries": [
        "US",
        "CA"
      ],
      "id": "72e724be-5121-5d5d-ab50-1ba7296bf2dd",
      "kind": "TOLL_FREE",
      "label": "+1 (833) … (US toll-free)",
      "verified": true
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<SmsSender>(serde_json::json!({"countries":["DE","UK","FR"],"id":"b58c0df7-9372-505d-87f2-2b050e076e26","kind":"ALPHANUMERIC","label":"ACME (alphanumeric)","verified":true})).unwrap_or_default(), serde_json::from_value::<SmsSender>(serde_json::json!({"countries":["DE"],"id":"af48362e-151e-57dc-bdbc-56d8053ddaab","kind":"ALPHANUMERIC","label":"+49 157 … (German long number)","verified":true})).unwrap_or_default(), serde_json::from_value::<SmsSender>(serde_json::json!({"countries":["US","CA"],"id":"72e724be-5121-5d5d-ab50-1ba7296bf2dd","kind":"TOLL_FREE","label":"+1 (833) … (US toll-free)","verified":true})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
