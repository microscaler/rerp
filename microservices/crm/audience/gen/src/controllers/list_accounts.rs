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
    //       "country_code": "DE",
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "name": "Northwind Traders",
    //       "website": "https://northwind.example"
    //     },
    //     {
    //       "contact_count": 0,
    //       "country_code": "GB",
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "name": "Fabrikam Inc",
    //       "website": "https://fabrikam.example"
    //     },
    //     {
    //       "contact_count": 0,
    //       "country_code": "ZA",
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "name": "Contoso Ltd",
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
      "country_code": "DE",
      "create_date": "2026-11-05T16:00:00Z",
      "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
      "name": "Northwind Traders",
      "website": "https://northwind.example"
    },
    {
      "contact_count": 0,
      "country_code": "GB",
      "create_date": "2026-11-05T16:00:00Z",
      "id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "name": "Fabrikam Inc",
      "website": "https://fabrikam.example"
    },
    {
      "contact_count": 0,
      "country_code": "ZA",
      "create_date": "2026-11-05T16:00:00Z",
      "id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "name": "Contoso Ltd",
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
        items: vec![serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"country_code":"DE","create_date":"2026-11-05T16:00:00Z","id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","name":"Northwind Traders","website":"https://northwind.example"})).unwrap_or_default(), serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"country_code":"GB","create_date":"2026-11-05T16:00:00Z","id":"576a7630-a5b2-566c-ac00-caca59662cb0","name":"Fabrikam Inc","website":"https://fabrikam.example"})).unwrap_or_default(), serde_json::from_value::<Organisation>(serde_json::json!({"contact_count":0,"country_code":"ZA","create_date":"2026-11-05T16:00:00Z","id":"da09c739-6e09-5587-a658-88f6b9eea0bd","name":"Contoso Ltd","website":"https://contoso.example"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
