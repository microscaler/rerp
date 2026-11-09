// User-owned controller for handler 'convert_lead'.

use crate::handlers::convert_lead::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ConvertLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "conversion_id": "2c4a2628-c1ef-51f9-bb85-7af366714c87",
    //   "lead_id": "36ad9027-d2b6-5440-962c-1bf60325b115",
    //   "reason": "order",
    //   "stage": "Stage"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "conversion_id": "2c4a2628-c1ef-51f9-bb85-7af366714c87",
  "lead_id": "36ad9027-d2b6-5440-962c-1bf60325b115",
  "reason": "order",
  "stage": "Stage"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        conversion_id: "2c4a2628-c1ef-51f9-bb85-7af366714c87".to_string(),
        lead_id: "36ad9027-d2b6-5440-962c-1bf60325b115".to_string(),
        message_id: Some("example".to_string()),
        reason: "order".to_string(),
        stage: "Stage".to_string(),
    })
}
