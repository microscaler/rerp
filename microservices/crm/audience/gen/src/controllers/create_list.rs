// User-owned controller for handler 'create_list'.

use crate::handlers::create_list::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateListController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        confirmation_template_id: Some("example".to_string()),
        created_at: "example".to_string(),
        description: Some("example".to_string()),
        double_opt_in: true,
        id: "example".to_string(),
        is_archived: Some(true),
        name: "example".to_string(),
        sender_id: "example".to_string(),
        subscriber_count: Some(42),
        unsubscribed_count: Some(42),
        updated_at: Some("example".to_string()),
    })
}
