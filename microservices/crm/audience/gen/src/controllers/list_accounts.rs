// User-owned controller for handler 'list_accounts'.

use crate::handlers::list_accounts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Organisation;

#[handler(ListAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_count": 0,
    //       "converted": true,
    //       "country_code": "DE",
    //       "created_at": "2026-01-09T16:00:00Z",
    //       "domain": "northwindtraders.example",
    //       "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "industry": "Logistics",
    //       "name": "Northwind Traders",
    //       "open_deal_count": 0,
    //       "size": "1-10",
    //       "website": "https://northwind.example"
    //     },
    //     {
    //       "contact_count": 0,
    //       "converted": false,
    //       "country_code": "GB",
    //       "created_at": "2026-01-29T16:00:00Z",
    //       "domain": "fabrikam.example",
    //       "id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "industry": "Retail",
    //       "name": "Fabrikam Inc",
    //       "open_deal_count": 0,
    //       "size": "11-50",
    //       "website": "https://fabrikam.example"
    //     },
    //     {
    //       "contact_count": 0,
    //       "converted": false,
    //       "country_code": "ZA",
    //       "created_at": "2026-02-18T16:00:00Z",
    //       "domain": "contoso.example",
    //       "id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "industry": "Finance",
    //       "name": "Contoso Ltd",
    //       "open_deal_count": 0,
    //       "size": "51-200",
    //       "website": "https://contoso.example"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 12
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "contact_count": 0,
      "converted": true,
      "country_code": "DE",
      "created_at": "2026-01-09T16:00:00Z",
      "domain": "northwindtraders.example",
      "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
      "industry": "Logistics",
      "name": "Northwind Traders",
      "open_deal_count": 0,
      "size": "1-10",
      "website": "https://northwind.example"
    },
    {
      "contact_count": 0,
      "converted": false,
      "country_code": "GB",
      "created_at": "2026-01-29T16:00:00Z",
      "domain": "fabrikam.example",
      "id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "industry": "Retail",
      "name": "Fabrikam Inc",
      "open_deal_count": 0,
      "size": "11-50",
      "website": "https://fabrikam.example"
    },
    {
      "contact_count": 0,
      "converted": false,
      "country_code": "ZA",
      "created_at": "2026-02-18T16:00:00Z",
      "domain": "contoso.example",
      "id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "industry": "Finance",
      "name": "Contoso Ltd",
      "open_deal_count": 0,
      "size": "51-200",
      "website": "https://contoso.example"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 12
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"converted":true,"country_code":"DE","created_at":"2026-01-09T16:00:00Z","domain":"northwindtraders.example","id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","industry":"Logistics","name":"Northwind Traders","open_deal_count":0,"size":"1-10","website":"https://northwind.example"})).unwrap_or_default(), serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"converted":false,"country_code":"GB","created_at":"2026-01-29T16:00:00Z","domain":"fabrikam.example","id":"576a7630-a5b2-566c-ac00-caca59662cb0","industry":"Retail","name":"Fabrikam Inc","open_deal_count":0,"size":"11-50","website":"https://fabrikam.example"})).unwrap_or_default(), serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"converted":false,"country_code":"ZA","created_at":"2026-02-18T16:00:00Z","domain":"contoso.example","id":"da09c739-6e09-5587-a658-88f6b9eea0bd","industry":"Finance","name":"Contoso Ltd","open_deal_count":0,"size":"51-200","website":"https://contoso.example"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
