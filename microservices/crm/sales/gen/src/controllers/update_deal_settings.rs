// User-owned controller for handler 'update_deal_settings'.

use crate::handlers::update_deal_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateDealSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        default_currency: Some("example".to_string()),
        default_pipeline_id: Some("example".to_string()),
        require_expected_close_from_stage_id: Some("example".to_string()),
        win_probability: Some(Default::default()),
    })
}
