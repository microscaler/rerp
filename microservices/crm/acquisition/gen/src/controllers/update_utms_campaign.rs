// User-owned controller for handler 'update_utms_campaign'.

use crate::handlers::update_utms_campaign::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateUtmsCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": true,
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "name": "Northwind Traders",
    //   "organisation_id": "265b0f8c-dac8-5c05-a3b9-2ac0ffc65dd1",
    //   "sequence": 1
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "active": true,
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "name": "Northwind Traders",
  "organisation_id": "265b0f8c-dac8-5c05-a3b9-2ac0ffc65dd1",
  "sequence": 1
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: Some(true),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        name: "Northwind Traders".to_string(),
        organisation_id: Some("265b0f8c-dac8-5c05-a3b9-2ac0ffc65dd1".to_string()),
        sequence: Some(1),
    })
}
