// User-owned controller for handler 'list_campaign_report_contacts'.

use crate::handlers::list_campaign_report_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CampaignReportContact;

#[handler(ListCampaignReportContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
    //       "email": "Email",
    //       "status": "DELIVERED"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
      "email": "Email",
      "status": "DELIVERED"
    }
  ],
  "limit": 25,
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
        items: vec![serde_json::from_value::<CampaignReportContact>(serde_json::json!({"contact_id":"c891a339-996d-56b9-a022-244c570f48c9","email":"Email","status":"DELIVERED"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
