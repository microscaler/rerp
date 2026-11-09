// User-owned controller for handler 'get_email_client_trends'.

use crate::handlers::get_email_client_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EmailClientRow;
#[allow(unused_imports)]
use crate::handlers::types::NameCount;

#[handler(GetEmailClientTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_family": [
    //     {
    //       "count": 58,
    //       "name": "WEBMAIL"
    //     },
    //     {
    //       "count": 31,
    //       "name": "MOBILE"
    //     },
    //     {
    //       "count": 9,
    //       "name": "DESKTOP"
    //     }
    //   ],
    //   "clients": [
    //     {
    //       "client": "Gmail",
    //       "family": "WEBMAIL",
    //       "opens": 9840,
    //       "share_pct": 40
    //     },
    //     {
    //       "client": "Apple Mail",
    //       "family": "DESKTOP",
    //       "opens": 5120,
    //       "share_pct": 21
    //     },
    //     {
    //       "client": "iOS Mail",
    //       "family": "MOBILE",
    //       "opens": 3210,
    //       "share_pct": 13
    //     }
    //   ],
    //   "total_opens": 24466
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "by_family": [
    {
      "count": 58,
      "name": "WEBMAIL"
    },
    {
      "count": 31,
      "name": "MOBILE"
    },
    {
      "count": 9,
      "name": "DESKTOP"
    }
  ],
  "clients": [
    {
      "client": "Gmail",
      "family": "WEBMAIL",
      "opens": 9840,
      "share_pct": 40
    },
    {
      "client": "Apple Mail",
      "family": "DESKTOP",
      "opens": 5120,
      "share_pct": 21
    },
    {
      "client": "iOS Mail",
      "family": "MOBILE",
      "opens": 3210,
      "share_pct": 13
    }
  ],
  "total_opens": 24466
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_family: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":58,"name":"WEBMAIL"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":31,"name":"MOBILE"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":9,"name":"DESKTOP"})).unwrap_or_default()]),clients: vec![serde_json::from_value::<EmailClientRow>(serde_json::json!({"client":"Gmail","family":"WEBMAIL","opens":9840,"share_pct":40})).unwrap_or_default(), serde_json::from_value::<EmailClientRow>(serde_json::json!({"client":"Apple Mail","family":"DESKTOP","opens":5120,"share_pct":21})).unwrap_or_default(), serde_json::from_value::<EmailClientRow>(serde_json::json!({"client":"iOS Mail","family":"MOBILE","opens":3210,"share_pct":13})).unwrap_or_default()],total_opens: 24466,
    })
}
