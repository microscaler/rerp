// User-owned controller for handler 'upload_deal_file'.

use crate::handlers::upload_deal_file::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UploadDealFileController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "filename": "Filename",
    //   "id": "93fa29a9-2114-561b-bc28-f28f2a42613a",
    //   "size": 3,
    //   "uploaded_by": "Me"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-11-05T16:00:00Z",
  "filename": "Filename",
  "id": "93fa29a9-2114-561b-bc28-f28f2a42613a",
  "size": 3,
  "uploaded_by": "Me"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        content_type: Some("example".to_string()),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        filename: "Filename".to_string(),
        id: "93fa29a9-2114-561b-bc28-f28f2a42613a".to_string(),
        size: 3,
        uploaded_by: Some("Me".to_string()),
        url: Some("example".to_string()),
    })
}
