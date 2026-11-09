// User-owned controller for handler 'list_tracking_domains'.

use crate::handlers::list_tracking_domains::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TrackingDomain;

#[handler(ListTrackingDomainsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "domain": "mail.acme.example",
    //       "id": "7b8f77a3-8b75-51bb-a3b9-b7267f501814",
    //       "last_checked_at": "2026-11-05T15:56:00Z",
    //       "records": [
    //         {
    //           "name": "mail.acme.example",
    //           "type": "CNAME",
    //           "value": "edge.crm.example.net",
    //           "verified": true
    //         },
    //         {
    //           "name": "_crm-verify.mail.acme.example",
    //           "type": "TXT",
    //           "value": "crm-verify=7c1e…",
    //           "verified": true
    //         }
    //       ],
    //       "status": "VERIFIED",
    //       "tls": true
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 1
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "domain": "mail.acme.example",
      "id": "7b8f77a3-8b75-51bb-a3b9-b7267f501814",
      "last_checked_at": "2026-11-05T15:56:00Z",
      "records": [
        {
          "name": "mail.acme.example",
          "type": "CNAME",
          "value": "edge.crm.example.net",
          "verified": true
        },
        {
          "name": "_crm-verify.mail.acme.example",
          "type": "TXT",
          "value": "crm-verify=7c1e…",
          "verified": true
        }
      ],
      "status": "VERIFIED",
      "tls": true
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 1
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<TrackingDomain>(serde_json::json!({"domain":"mail.acme.example","id":"7b8f77a3-8b75-51bb-a3b9-b7267f501814","last_checked_at":"2026-11-05T15:56:00Z","records":[{"name":"mail.acme.example","type":"CNAME","value":"edge.crm.example.net","verified":true},{"name":"_crm-verify.mail.acme.example","type":"TXT","value":"crm-verify=7c1e…","verified":true}],"status":"VERIFIED","tls":true})).unwrap_or_default()],limit: 25,page: 1,total: 1,
    })
}
