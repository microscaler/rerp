// User-owned controller for handler 'get_form'.

use crate::handlers::get_form::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FormAction;
#[allow(unused_imports)]
use crate::handlers::types::FormFieldDef;
#[allow(unused_imports)]
use crate::handlers::types::FormStyle;

#[handler(GetFormController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        actions: Some(vec![]),
        allow_blank_overwrite: Some(true),
        allowed_origins: Some(vec![]),
        api_endpoint: Some("example".to_string()),
        block_free_mail: Some(true),
        blocked_domains: Some(vec![]),
        capture_fields: Default::default(),
        consent_required: Some(true),
        consent_text: Some("example".to_string()),
        conversion_rate: Some(1.5),
        created_at: Some("example".to_string()),
        custom_html: Some("example".to_string()),
        description: Some("example".to_string()),
        double_opt_in: Some(true),
        embed_code: Some("example".to_string()),
        fields: Some(vec![]),
        id: "example".to_string(),
        is_active: Some(true),
        name: "example".to_string(),
        on_submit: Some("example".to_string()),
        public_url: Some("example".to_string()),
        redirect_url: Some("example".to_string()),
        style: Some(Default::default()),
        submission_count: Some(42),
        team_id: Some("example".to_string()),
        user_id: Some("example".to_string()),
        utm_campaign_id: Some("example".to_string()),
        utm_medium_id: Some("example".to_string()),
        utm_source_id: Some("example".to_string()),
        workflow_ids: Some(vec![]),
    })
}
