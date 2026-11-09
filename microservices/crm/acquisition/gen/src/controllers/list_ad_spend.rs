// User-owned controller for handler 'list_ad_spend'.

use crate::handlers::list_ad_spend::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdSpendRow;

#[handler(ListAdSpendController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "campaign": "BF early access · warm-up",
    //       "clicks": 27,
    //       "currency": "EUR",
    //       "date": "2026-11-05",
    //       "imported_at": "2026-11-05T10:00:00Z",
    //       "impressions": 8640,
    //       "platform": "META",
    //       "spend": "180.00"
    //     },
    //     {
    //       "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
    //       "campaign": "Brand search",
    //       "clicks": 8,
    //       "currency": "EUR",
    //       "date": "2026-11-05",
    //       "imported_at": "2026-11-05T10:00:00Z",
    //       "impressions": 2592,
    //       "platform": "GOOGLE_ADS",
    //       "spend": "54.00"
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "campaign": "Webinar invite · warm-up",
    //       "clicks": 9,
    //       "currency": "EUR",
    //       "date": "2026-11-05",
    //       "imported_at": "2026-11-05T10:00:00Z",
    //       "impressions": 2880,
    //       "platform": "LINKEDIN",
    //       "spend": "60.00"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 21
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "campaign": "BF early access · warm-up",
      "clicks": 27,
      "currency": "EUR",
      "date": "2026-11-05",
      "imported_at": "2026-11-05T10:00:00Z",
      "impressions": 8640,
      "platform": "META",
      "spend": "180.00"
    },
    {
      "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
      "campaign": "Brand search",
      "clicks": 8,
      "currency": "EUR",
      "date": "2026-11-05",
      "imported_at": "2026-11-05T10:00:00Z",
      "impressions": 2592,
      "platform": "GOOGLE_ADS",
      "spend": "54.00"
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "campaign": "Webinar invite · warm-up",
      "clicks": 9,
      "currency": "EUR",
      "date": "2026-11-05",
      "imported_at": "2026-11-05T10:00:00Z",
      "impressions": 2880,
      "platform": "LINKEDIN",
      "spend": "60.00"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 21
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","campaign":"BF early access · warm-up","clicks":27,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":8640,"platform":"META","spend":"180.00"})).unwrap_or_default(), serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","campaign":"Brand search","clicks":8,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":2592,"platform":"GOOGLE_ADS","spend":"54.00"})).unwrap_or_default(), serde_json::from_value::<AdSpendRow>(serde_json::json!({"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","campaign":"Webinar invite · warm-up","clicks":9,"currency":"EUR","date":"2026-11-05","imported_at":"2026-11-05T10:00:00Z","impressions":2880,"platform":"LINKEDIN","spend":"60.00"})).unwrap_or_default()],limit: 25,page: 1,total: 21,
    })
}
