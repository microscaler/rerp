// User-owned controller for handler 'retry_ad_conversion'.

use crate::handlers::retry_ad_conversion::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RetryAdConversionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: Some("example".to_string()),
        click_id_param: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        contact_name: Some("example".to_string()),
        currency: Some("example".to_string()),
        deal_id: Some("example".to_string()),
        error: Some("example".to_string()),
        event: Some("example".to_string()),
        external_event_name: Some("example".to_string()),
        id: "example".to_string(),
        identifiers_sent: Some(vec![]),
        occurred_at: Some("example".to_string()),
        platform: Some("example".to_string()),
        platform_response: Some("example".to_string()),
        rule_id: Some("example".to_string()),
        sent_at: Some("example".to_string()),
        status: "example".to_string(),
        value: Some(1.5),
    })
}
