// User-owned controller for handler 'list_ad_campaigns'.

use crate::handlers::list_ad_campaigns::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdCampaignRow;

#[handler(ListAdCampaignsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "attributed": {
    //         "cac": "1460.00",
    //         "deals": 3,
    //         "leads": 9,
    //         "revenue": "2400.00",
    //         "roas": 1.6,
    //         "won": 1
    //       },
    //       "clicks": 214,
    //       "cpc": "6.82",
    //       "external_id": "262ac3c8",
    //       "impressions": 71000,
    //       "level": "CAMPAIGN",
    //       "ltv_90d": "760.00",
    //       "name": "BF early access · warm-up",
    //       "platform": "Meta",
    //       "platform_reported": {
    //         "conversions": 11,
    //         "revenue": "9800.00"
    //       },
    //       "recommendation": "HOLD",
    //       "spend": "1460.00",
    //       "status": "ACTIVE"
    //     },
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "attributed": {
    //         "cac": "124.00",
    //         "deals": 8,
    //         "leads": 21,
    //         "revenue": "3900.00",
    //         "roas": 6.3,
    //         "won": 5
    //       },
    //       "clicks": 402,
    //       "cpc": "1.54",
    //       "external_id": "f3cd6604",
    //       "impressions": 38000,
    //       "level": "CAMPAIGN",
    //       "ltv_90d": "1240.00",
    //       "name": "Retarget · pricing visitors",
    //       "platform": "Meta + Google",
    //       "platform_reported": {
    //         "conversions": 14,
    //         "revenue": "12100.00"
    //       },
    //       "recommendation": "SCALE",
    //       "spend": "620.00",
    //       "status": "ACTIVE"
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "attributed": {
    //         "cac": "300.00",
    //         "deals": 5,
    //         "leads": 14,
    //         "revenue": "1400.00",
    //         "roas": 2.3,
    //         "won": 2
    //       },
    //       "clicks": 118,
    //       "cpc": "5.08",
    //       "external_id": "88429ed5",
    //       "impressions": 22000,
    //       "level": "CAMPAIGN",
    //       "ltv_90d": "2010.00",
    //       "name": "Webinar invite · warm-up",
    //       "platform": "LinkedIn",
    //       "platform_reported": {
    //         "conversions": 6,
    //         "revenue": "4200.00"
    //       },
    //       "recommendation": "HOLD",
    //       "spend": "600.00",
    //       "status": "ACTIVE"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 6
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "attributed": {
        "cac": "1460.00",
        "deals": 3,
        "leads": 9,
        "revenue": "2400.00",
        "roas": 1.6,
        "won": 1
      },
      "clicks": 214,
      "cpc": "6.82",
      "external_id": "262ac3c8",
      "impressions": 71000,
      "level": "CAMPAIGN",
      "ltv_90d": "760.00",
      "name": "BF early access · warm-up",
      "platform": "Meta",
      "platform_reported": {
        "conversions": 11,
        "revenue": "9800.00"
      },
      "recommendation": "HOLD",
      "spend": "1460.00",
      "status": "ACTIVE"
    },
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "attributed": {
        "cac": "124.00",
        "deals": 8,
        "leads": 21,
        "revenue": "3900.00",
        "roas": 6.3,
        "won": 5
      },
      "clicks": 402,
      "cpc": "1.54",
      "external_id": "f3cd6604",
      "impressions": 38000,
      "level": "CAMPAIGN",
      "ltv_90d": "1240.00",
      "name": "Retarget · pricing visitors",
      "platform": "Meta + Google",
      "platform_reported": {
        "conversions": 14,
        "revenue": "12100.00"
      },
      "recommendation": "SCALE",
      "spend": "620.00",
      "status": "ACTIVE"
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "attributed": {
        "cac": "300.00",
        "deals": 5,
        "leads": 14,
        "revenue": "1400.00",
        "roas": 2.3,
        "won": 2
      },
      "clicks": 118,
      "cpc": "5.08",
      "external_id": "88429ed5",
      "impressions": 22000,
      "level": "CAMPAIGN",
      "ltv_90d": "2010.00",
      "name": "Webinar invite · warm-up",
      "platform": "LinkedIn",
      "platform_reported": {
        "conversions": 6,
        "revenue": "4200.00"
      },
      "recommendation": "HOLD",
      "spend": "600.00",
      "status": "ACTIVE"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 6
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdCampaignRow>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","attributed":{"cac":"1460.00","deals":3,"leads":9,"revenue":"2400.00","roas":1.6,"won":1},"clicks":214,"cpc":"6.82","external_id":"262ac3c8","impressions":71000,"level":"CAMPAIGN","ltv_90d":"760.00","name":"BF early access · warm-up","platform":"Meta","platform_reported":{"conversions":11,"revenue":"9800.00"},"recommendation":"HOLD","spend":"1460.00","status":"ACTIVE"})).unwrap_or_default(), serde_json::from_value::<AdCampaignRow>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","attributed":{"cac":"124.00","deals":8,"leads":21,"revenue":"3900.00","roas":6.3,"won":5},"clicks":402,"cpc":"1.54","external_id":"f3cd6604","impressions":38000,"level":"CAMPAIGN","ltv_90d":"1240.00","name":"Retarget · pricing visitors","platform":"Meta + Google","platform_reported":{"conversions":14,"revenue":"12100.00"},"recommendation":"SCALE","spend":"620.00","status":"ACTIVE"})).unwrap_or_default(), serde_json::from_value::<AdCampaignRow>(serde_json::json!({"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","attributed":{"cac":"300.00","deals":5,"leads":14,"revenue":"1400.00","roas":2.3,"won":2},"clicks":118,"cpc":"5.08","external_id":"88429ed5","impressions":22000,"level":"CAMPAIGN","ltv_90d":"2010.00","name":"Webinar invite · warm-up","platform":"LinkedIn","platform_reported":{"conversions":6,"revenue":"4200.00"},"recommendation":"HOLD","spend":"600.00","status":"ACTIVE"})).unwrap_or_default()],limit: 25,page: 1,total: 6,
    })
}
