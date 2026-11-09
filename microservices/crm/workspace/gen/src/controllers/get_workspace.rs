// User-owned controller for handler 'get_workspace'.

use crate::handlers::get_workspace::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWorkspaceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "brand_domain": "Brand domain",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "default_currency": "EUR",
    //   "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //   "my_role": "owner",
    //   "name": "Northwind Traders",
    //   "organisation_id": "20df1243-e229-5084-b57d-e48d795731f8",
    //   "slug": "northwind",
    //   "timezone": "Europe/Berlin"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "brand_domain": "Brand domain",
  "created_at": "2026-11-05T16:00:00Z",
  "default_currency": "EUR",
  "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
  "my_role": "owner",
  "name": "Northwind Traders",
  "organisation_id": "20df1243-e229-5084-b57d-e48d795731f8",
  "slug": "northwind",
  "timezone": "Europe/Berlin"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        brand_domain: Some("Brand domain".to_string()),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        default_currency: Some("EUR".to_string()),
        id: "94b293ed-258b-5091-8e79-75d2ac471f25".to_string(),
        my_role: Some("owner".to_string()),
        name: "Northwind Traders".to_string(),
        organisation_id: "20df1243-e229-5084-b57d-e48d795731f8".to_string(),
        slug: Some("northwind".to_string()),
        timezone: Some("Europe/Berlin".to_string()),
    })
}
