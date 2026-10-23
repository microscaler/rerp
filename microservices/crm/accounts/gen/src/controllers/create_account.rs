// User-owned controller for handler 'create_account'.

use crate::handlers::create_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AccountOrigin;

#[handler(CreateAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: Some(true),
        active_lead_count: Some(42),
        annual_revenue: Some(1.5),
        child_ids: Some(vec![]),
        city: Some("example".to_string()),
        color: Some(42),
        company_size: Some("example".to_string()),
        contact_count: Some(42),
        contact_name: Some("example".to_string()),
        country_code: Some("example".to_string()),
        country_id: Some("example".to_string()),
        country_name: Some("example".to_string()),
        create_date: "example".to_string(),
        description: Some("example".to_string()),
        email: Some("example".to_string()),
        employees: Some(42),
        id: "example".to_string(),
        industry_id: Some("example".to_string()),
        industry_name: Some("example".to_string()),
        name: "example".to_string(),
        origin: Some(Default::default()),
        parent_id: Some("example".to_string()),
        phone: Some("example".to_string()),
        region: Some("example".to_string()),
        state_id: Some("example".to_string()),
        street: Some("example".to_string()),
        street2: Some("example".to_string()),
        tax_id: Some("example".to_string()),
        total_revenue: Some(1.5),
        website: Some("example".to_string()),
        write_date: Some("example".to_string()),
        zip: Some("example".to_string()),
    })
}
