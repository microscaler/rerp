// User-owned controller for handler 'list_deal_files'.

use crate::handlers::list_deal_files::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Attachment;

#[handler(ListDealFilesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-04T11:00:00Z",
    //       "filename": "Filename",
    //       "id": "d8f2aca9-2c33-5bb8-9b2a-3aba9a6b7ca6",
    //       "size": 3,
    //       "uploaded_by": "Alex Morgan",
    //       "url": "https://crm.acme.example/files/dfile1"
    //     },
    //     {
    //       "created_at": "2026-11-04T13:00:00Z",
    //       "filename": "Filename",
    //       "id": "9c808bcb-a18d-53f5-9805-02247a239a42",
    //       "size": 3,
    //       "uploaded_by": "Alex Morgan",
    //       "url": "https://crm.acme.example/files/dfile2"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "created_at": "2026-11-04T11:00:00Z",
      "filename": "Filename",
      "id": "d8f2aca9-2c33-5bb8-9b2a-3aba9a6b7ca6",
      "size": 3,
      "uploaded_by": "Alex Morgan",
      "url": "https://crm.acme.example/files/dfile1"
    },
    {
      "created_at": "2026-11-04T13:00:00Z",
      "filename": "Filename",
      "id": "9c808bcb-a18d-53f5-9805-02247a239a42",
      "size": 3,
      "uploaded_by": "Alex Morgan",
      "url": "https://crm.acme.example/files/dfile2"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Attachment>(serde_json::json!({"created_at":"2026-11-04T11:00:00Z","filename":"Filename","id":"d8f2aca9-2c33-5bb8-9b2a-3aba9a6b7ca6","size":3,"uploaded_by":"Alex Morgan","url":"https://crm.acme.example/files/dfile1"})).unwrap_or_default(), serde_json::from_value::<Attachment>(serde_json::json!({"created_at":"2026-11-04T13:00:00Z","filename":"Filename","id":"9c808bcb-a18d-53f5-9805-02247a239a42","size":3,"uploaded_by":"Alex Morgan","url":"https://crm.acme.example/files/dfile2"})).unwrap_or_default()],limit: 20,page: 1,total: 2,
    })
}
