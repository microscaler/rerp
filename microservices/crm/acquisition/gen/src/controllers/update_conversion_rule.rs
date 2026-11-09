// User-owned controller for handler 'update_conversion_rule'.

use crate::handlers::update_conversion_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateConversionRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: "example".to_string(),
        consent_required: Some(true),
        currency: Some("example".to_string()),
        dedupe_key: Some("example".to_string()),
        enabled: Some(true),
        event: "example".to_string(),
        event_filter: Some(Default::default()),
        external_event_name: Some("example".to_string()),
        failed_30d: Some(42),
        fixed_value: Some(1.5),
        id: Some("example".to_string()),
        name: Some("example".to_string()),
        require_click: Some(true),
        sent_30d: Some(42),
        value_from: Some("example".to_string()),
        window_days: Some(42),
    })
}
