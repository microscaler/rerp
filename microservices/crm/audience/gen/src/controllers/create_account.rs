// User-owned controller for handler 'create_account'.

use crate::handlers::create_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_count": 0,
    //   "converted": true,
    //   "country_code": "DE",
    //   "created_at": "2026-01-09T16:00:00Z",
    //   "domain": "northwindtraders.example",
    //   "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "industry": "Logistics",
    //   "name": "Northwind Traders",
    //   "open_deal_count": 0,
    //   "size": "1-10",
    //   "website": "https://northwind.example"
    // }
    match serde_json::from_str::<Response>(
        r#"{
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
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contact_count: Some(0),
        converted: Some(true),
        country_code: Some("DE".to_string()),
        created_at: "2026-01-09T16:00:00Z".to_string(),
        customer_account_id: Some("example".to_string()),
        domain: Some("northwindtraders.example".to_string()),
        id: "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string(),
        industry: Some("Logistics".to_string()),
        name: "Northwind Traders".to_string(),
        open_deal_count: Some(0),
        parent_id: Some("example".to_string()),
        size: Some("1-10".to_string()),
        updated_at: Some("example".to_string()),
        website: Some("https://northwind.example".to_string()),
    })
}
