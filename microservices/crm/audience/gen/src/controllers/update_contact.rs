// User-owned controller for handler 'update_contact'.

use crate::handlers::update_contact::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateContactController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: Some(true),
        child_contact_ids: Some(vec![]),
        city: Some("example".to_string()),
        color: Some(42),
        company_id: Some("example".to_string()),
        country_id: Some("example".to_string()),
        create_date: "example".to_string(),
        custom_fields: Some(Default::default()),
        department: Some("example".to_string()),
        email: Some("example".to_string()),
        function: Some("example".to_string()),
        id: "example".to_string(),
        is_company: Some(true),
        last_activity_at: Some("example".to_string()),
        lead_count: Some(42),
        mobile: Some("example".to_string()),
        name: "example".to_string(),
        opportunity_count: Some(42),
        parent_id: Some("example".to_string()),
        phone: Some("example".to_string()),
        score: Some(42),
        state_id: Some("example".to_string()),
        street: Some("example".to_string()),
        street2: Some("example".to_string()),
        tags: Some(vec![]),
        timezone: Some("example".to_string()),
        title: Some("example".to_string()),
        user_ids: Some(vec![]),
        write_date: Some("example".to_string()),
        zip: Some("example".to_string()),
    })
}
