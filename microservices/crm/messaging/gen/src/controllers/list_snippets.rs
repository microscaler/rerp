// User-owned controller for handler 'list_snippets'.

use crate::handlers::list_snippets::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Snippet;

#[handler(ListSnippetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "d2784443-bc70-5675-8a68-30620c50a0eb",
    //       "key": "snippet.signature",
    //       "used_by": 6,
    //       "value": "Alex Morgan · Head of Marketing · acme.example"
    //     },
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "d2e3e75f-72fa-569c-b2f1-97bedbfcd9af",
    //       "key": "snippet.support_hours",
    //       "used_by": 3,
    //       "value": "Mon–Fri 09:00–18:00 CET"
    //     },
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "a4768040-ea68-594e-9a89-fa56cf20c045",
    //       "key": "snippet.legal_footer",
    //       "used_by": 11,
    //       "value": "Acme Co, 12 Example Street, 10115 Berlin · VAT DE123456789"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "d2784443-bc70-5675-8a68-30620c50a0eb",
      "key": "snippet.signature",
      "used_by": 6,
      "value": "Alex Morgan · Head of Marketing · acme.example"
    },
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "d2e3e75f-72fa-569c-b2f1-97bedbfcd9af",
      "key": "snippet.support_hours",
      "used_by": 3,
      "value": "Mon–Fri 09:00–18:00 CET"
    },
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "a4768040-ea68-594e-9a89-fa56cf20c045",
      "key": "snippet.legal_footer",
      "used_by": 11,
      "value": "Acme Co, 12 Example Street, 10115 Berlin · VAT DE123456789"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Snippet>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"d2784443-bc70-5675-8a68-30620c50a0eb","key":"snippet.signature","used_by":6,"value":"Alex Morgan · Head of Marketing · acme.example"})).unwrap_or_default(), serde_json::from_value::<Snippet>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"d2e3e75f-72fa-569c-b2f1-97bedbfcd9af","key":"snippet.support_hours","used_by":3,"value":"Mon–Fri 09:00–18:00 CET"})).unwrap_or_default(), serde_json::from_value::<Snippet>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"a4768040-ea68-594e-9a89-fa56cf20c045","key":"snippet.legal_footer","used_by":11,"value":"Acme Co, 12 Example Street, 10115 Berlin · VAT DE123456789"})).unwrap_or_default()],limit: 25,page: 1,total: 4,
    })
}
