// User-owned controller for handler 'create_asset'.

use crate::handlers::create_asset::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Asset;

#[handler(CreateAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "asset": {
    //     "content_type": "Content type",
    //     "created_at": "2026-11-05T16:00:00Z",
    //     "height": 3,
    //     "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //     "name": "Northwind Traders",
    //     "size_bytes": 3,
    //     "url": "https://northwindtraders.example/pricing",
    //     "width": 3
    //   },
    //   "expires_at": "2026-11-05T16:00:00Z",
    //   "upload_url": "https://northwindtraders.example/pricing"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "asset": {
    "content_type": "Content type",
    "created_at": "2026-11-05T16:00:00Z",
    "height": 3,
    "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    "name": "Northwind Traders",
    "size_bytes": 3,
    "url": "https://northwindtraders.example/pricing",
    "width": 3
  },
  "expires_at": "2026-11-05T16:00:00Z",
  "upload_url": "https://northwindtraders.example/pricing"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        asset: serde_json::from_value::<Asset>(serde_json::json!({"content_type":"Content type","created_at":"2026-11-05T16:00:00Z","height":3,"id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","name":"Northwind Traders","size_bytes":3,"url":"https://northwindtraders.example/pricing","width":3})).unwrap_or_default(),expires_at: "2026-11-05T16:00:00Z".to_string(),upload_url: "https://northwindtraders.example/pricing".to_string(),
    })
}
