// User-owned controller for handler 'import_ad_spend'.

use crate::handlers::import_ad_spend::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ImportAdSpendController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //   "status": "QUEUED"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
  "status": "QUEUED"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        account_id: Some("example".to_string()),
        error: Some("example".to_string()),
        finished_at: Some("example".to_string()),
        from: Some("example".to_string()),
        id: Some("ded95315-6c77-51a0-b8ea-66b76fada21a".to_string()),
        rows: Some(42),
        status: Some("QUEUED".to_string()),
        to: Some("example".to_string()),
    })
}
