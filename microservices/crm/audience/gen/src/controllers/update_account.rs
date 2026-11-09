// User-owned controller for handler 'update_account'.

use crate::handlers::update_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::OrganisationOrigin;

#[handler(UpdateAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_count": 0,
    //   "country_code": "DE",
    //   "create_date": "2026-11-05T16:00:00Z",
    //   "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "name": "Northwind Traders",
    //   "website": "https://northwind.example"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contact_count": 0,
  "country_code": "DE",
  "create_date": "2026-11-05T16:00:00Z",
  "id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
  "name": "Northwind Traders",
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
        active: Some(true),
        active_lead_count: Some(42),
        annual_revenue: Some("example".to_string()),
        child_ids: Some(vec![]),
        city: Some("example".to_string()),
        color: Some(42),
        company_size: Some("example".to_string()),
        contact_count: Some(0),
        contact_name: Some("example".to_string()),
        country_code: Some("DE".to_string()),
        country_id: Some("example".to_string()),
        country_name: Some("example".to_string()),
        create_date: "2026-11-05T16:00:00Z".to_string(),
        description: Some("example".to_string()),
        email: Some("example".to_string()),
        employees: Some(42),
        id: "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string(),
        industry_id: Some("example".to_string()),
        industry_name: Some("example".to_string()),
        name: "Northwind Traders".to_string(),
        origin: Some(Default::default()),
        parent_id: Some("example".to_string()),
        phone: Some("example".to_string()),
        region: Some("example".to_string()),
        state_id: Some("example".to_string()),
        street: Some("example".to_string()),
        street2: Some("example".to_string()),
        tax_id: Some("example".to_string()),
        total_revenue: Some("example".to_string()),
        website: Some("https://northwind.example".to_string()),
        write_date: Some("example".to_string()),
        zip: Some("example".to_string()),
    })
}
