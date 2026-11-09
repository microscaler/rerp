// User-owned controller for handler 'list_contact_sends'.

use crate::handlers::list_contact_sends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Send;

#[handler(ListContactSendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //       "campaign_name": "October product update",
    //       "channel": "EMAIL",
    //       "clicked_at": "2026-10-28T16:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-10-28T16:00:00Z",
    //       "id": "540e3991-1a5a-53f5-b5d2-40847552e9b8",
    //       "opened_at": "2026-10-28T16:00:00Z",
    //       "status": "DELIVERED",
    //       "subject": "October product update",
    //       "to": "priya.natarajan@northwindtraders.example"
    //     },
    //     {
    //       "campaign_id": "cd847d34-2090-5e40-afc2-06c94055cc85",
    //       "campaign_name": "Webinar invite · pricing strategy",
    //       "channel": "EMAIL",
    //       "clicked_at": "2026-11-05T16:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-10-08T16:00:00Z",
    //       "id": "83e7787e-deb9-5ece-93c5-58284dd9ceb5",
    //       "opened_at": "2026-10-08T16:00:00Z",
    //       "status": "DELIVERED",
    //       "subject": "Webinar invite · pricing strategy",
    //       "to": "priya.natarajan@northwindtraders.example"
    //     },
    //     {
    //       "campaign_id": "dcad966f-24e6-5a17-bb05-807224d4fe09",
    //       "campaign_name": "September product update",
    //       "channel": "EMAIL",
    //       "clicked_at": "2026-11-05T16:00:00Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-07-20T16:00:00Z",
    //       "id": "f4276478-8eee-5e00-ae6b-d72dff226d41",
    //       "opened_at": "2026-07-20T16:00:00Z",
    //       "status": "DELIVERED",
    //       "subject": "October product update",
    //       "to": "priya.natarajan@northwindtraders.example"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
      "campaign_name": "October product update",
      "channel": "EMAIL",
      "clicked_at": "2026-10-28T16:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "created_at": "2026-10-28T16:00:00Z",
      "id": "540e3991-1a5a-53f5-b5d2-40847552e9b8",
      "opened_at": "2026-10-28T16:00:00Z",
      "status": "DELIVERED",
      "subject": "October product update",
      "to": "priya.natarajan@northwindtraders.example"
    },
    {
      "campaign_id": "cd847d34-2090-5e40-afc2-06c94055cc85",
      "campaign_name": "Webinar invite · pricing strategy",
      "channel": "EMAIL",
      "clicked_at": "2026-11-05T16:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "created_at": "2026-10-08T16:00:00Z",
      "id": "83e7787e-deb9-5ece-93c5-58284dd9ceb5",
      "opened_at": "2026-10-08T16:00:00Z",
      "status": "DELIVERED",
      "subject": "Webinar invite · pricing strategy",
      "to": "priya.natarajan@northwindtraders.example"
    },
    {
      "campaign_id": "dcad966f-24e6-5a17-bb05-807224d4fe09",
      "campaign_name": "September product update",
      "channel": "EMAIL",
      "clicked_at": "2026-11-05T16:00:00Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "created_at": "2026-07-20T16:00:00Z",
      "id": "f4276478-8eee-5e00-ae6b-d72dff226d41",
      "opened_at": "2026-07-20T16:00:00Z",
      "status": "DELIVERED",
      "subject": "October product update",
      "to": "priya.natarajan@northwindtraders.example"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"7de39bcc-4bc6-551b-a967-7b1629b846be","campaign_name":"October product update","channel":"EMAIL","clicked_at":"2026-10-28T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-28T16:00:00Z","id":"540e3991-1a5a-53f5-b5d2-40847552e9b8","opened_at":"2026-10-28T16:00:00Z","status":"DELIVERED","subject":"October product update","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"cd847d34-2090-5e40-afc2-06c94055cc85","campaign_name":"Webinar invite · pricing strategy","channel":"EMAIL","clicked_at":"2026-11-05T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-10-08T16:00:00Z","id":"83e7787e-deb9-5ece-93c5-58284dd9ceb5","opened_at":"2026-10-08T16:00:00Z","status":"DELIVERED","subject":"Webinar invite · pricing strategy","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"campaign_id":"dcad966f-24e6-5a17-bb05-807224d4fe09","campaign_name":"September product update","channel":"EMAIL","clicked_at":"2026-11-05T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-07-20T16:00:00Z","id":"f4276478-8eee-5e00-ae6b-d72dff226d41","opened_at":"2026-07-20T16:00:00Z","status":"DELIVERED","subject":"October product update","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default()],limit: 20,page: 1,total: 3,
    })
}
