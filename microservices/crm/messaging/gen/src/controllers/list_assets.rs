// User-owned controller for handler 'list_assets'.

use crate::handlers::list_assets::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Asset;

#[handler(ListAssetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "height": 600,
    //       "id": "8692f757-7ab4-5792-8895-3cb42594a05d",
    //       "name": "hero-october.png",
    //       "size_bytes": 412000,
    //       "url": "https://mail.acme.example/assets/hero-october.png",
    //       "width": 1200
    //     },
    //     {
    //       "created_at": "2026-10-27T16:00:00Z",
    //       "height": 600,
    //       "id": "d016ee7e-b764-5145-80a9-19a23ccce65e",
    //       "name": "logo.png",
    //       "size_bytes": 18000,
    //       "url": "https://mail.acme.example/assets/logo.png",
    //       "width": 1200
    //     },
    //     {
    //       "created_at": "2026-10-18T16:00:00Z",
    //       "height": 600,
    //       "id": "83ff295d-5bfe-5dc1-acfe-3b615507475d",
    //       "name": "case-study-northwind.jpg",
    //       "size_bytes": 630000,
    //       "url": "https://mail.acme.example/assets/case-study-northwind.jpg",
    //       "width": 1200
    //     }
    //   ],
    //   "limit": 100,
    //   "page": 1,
    //   "total": 42
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "created_at": "2026-11-05T16:00:00Z",
      "height": 600,
      "id": "8692f757-7ab4-5792-8895-3cb42594a05d",
      "name": "hero-october.png",
      "size_bytes": 412000,
      "url": "https://mail.acme.example/assets/hero-october.png",
      "width": 1200
    },
    {
      "created_at": "2026-10-27T16:00:00Z",
      "height": 600,
      "id": "d016ee7e-b764-5145-80a9-19a23ccce65e",
      "name": "logo.png",
      "size_bytes": 18000,
      "url": "https://mail.acme.example/assets/logo.png",
      "width": 1200
    },
    {
      "created_at": "2026-10-18T16:00:00Z",
      "height": 600,
      "id": "83ff295d-5bfe-5dc1-acfe-3b615507475d",
      "name": "case-study-northwind.jpg",
      "size_bytes": 630000,
      "url": "https://mail.acme.example/assets/case-study-northwind.jpg",
      "width": 1200
    }
  ],
  "limit": 100,
  "page": 1,
  "total": 42
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","height":600,"id":"8692f757-7ab4-5792-8895-3cb42594a05d","name":"hero-october.png","size_bytes":412000,"url":"https://mail.acme.example/assets/hero-october.png","width":1200})).unwrap_or_default(), serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-10-27T16:00:00Z","height":600,"id":"d016ee7e-b764-5145-80a9-19a23ccce65e","name":"logo.png","size_bytes":18000,"url":"https://mail.acme.example/assets/logo.png","width":1200})).unwrap_or_default(), serde_json::from_value::<Asset>(serde_json::json!({"created_at":"2026-10-18T16:00:00Z","height":600,"id":"83ff295d-5bfe-5dc1-acfe-3b615507475d","name":"case-study-northwind.jpg","size_bytes":630000,"url":"https://mail.acme.example/assets/case-study-northwind.jpg","width":1200})).unwrap_or_default()],limit: 100,page: 1,total: 42,
    })
}
