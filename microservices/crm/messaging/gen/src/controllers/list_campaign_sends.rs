// User-owned controller for handler 'list_campaign_sends'.

use crate::handlers::list_campaign_sends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Send;

#[handler(ListCampaignSendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "channel": "EMAIL",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "01a1ef84-be9a-5166-aaa5-6a6a9902308c",
    //       "opened_at": "2026-10-28T16:00:00Z",
    //       "status": "DELIVERED",
    //       "to": "priya.natarajan@northwindtraders.example"
    //     },
    //     {
    //       "channel": "EMAIL",
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "294a11f7-a7ec-5743-be57-1a1f486a17b9",
    //       "opened_at": "2026-10-28T17:00:00Z",
    //       "status": "DELIVERED",
    //       "to": "tom.okafor@fabrikam.example"
    //     },
    //     {
    //       "channel": "EMAIL",
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "c84b4b32-0560-5476-99cf-1822e1da82fe",
    //       "opened_at": "2026-11-05T16:00:00Z",
    //       "status": "BOUNCED",
    //       "to": "lena.fischer@contoso.example"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 9870
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "channel": "EMAIL",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "created_at": "2026-11-05T16:00:00Z",
      "id": "01a1ef84-be9a-5166-aaa5-6a6a9902308c",
      "opened_at": "2026-10-28T16:00:00Z",
      "status": "DELIVERED",
      "to": "priya.natarajan@northwindtraders.example"
    },
    {
      "channel": "EMAIL",
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "created_at": "2026-11-05T16:00:00Z",
      "id": "294a11f7-a7ec-5743-be57-1a1f486a17b9",
      "opened_at": "2026-10-28T17:00:00Z",
      "status": "DELIVERED",
      "to": "tom.okafor@fabrikam.example"
    },
    {
      "channel": "EMAIL",
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "created_at": "2026-11-05T16:00:00Z",
      "id": "c84b4b32-0560-5476-99cf-1822e1da82fe",
      "opened_at": "2026-11-05T16:00:00Z",
      "status": "BOUNCED",
      "to": "lena.fischer@contoso.example"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 9870
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","created_at":"2026-11-05T16:00:00Z","id":"01a1ef84-be9a-5166-aaa5-6a6a9902308c","opened_at":"2026-10-28T16:00:00Z","status":"DELIVERED","to":"priya.natarajan@northwindtraders.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","created_at":"2026-11-05T16:00:00Z","id":"294a11f7-a7ec-5743-be57-1a1f486a17b9","opened_at":"2026-10-28T17:00:00Z","status":"DELIVERED","to":"tom.okafor@fabrikam.example"})).unwrap_or_default(), serde_json::from_value::<Send>(serde_json::json!({"channel":"EMAIL","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","created_at":"2026-11-05T16:00:00Z","id":"c84b4b32-0560-5476-99cf-1822e1da82fe","opened_at":"2026-11-05T16:00:00Z","status":"BOUNCED","to":"lena.fischer@contoso.example"})).unwrap_or_default()],limit: 20,page: 1,total: 9870,
    })
}
