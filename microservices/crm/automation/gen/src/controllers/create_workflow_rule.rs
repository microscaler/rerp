// User-owned controller for handler 'create_workflow_rule'.

use crate::handlers::create_workflow_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActionType;
#[allow(unused_imports)]
use crate::handlers::types::ConditionOperator;
#[allow(unused_imports)]
use crate::handlers::types::ConditionValueType;

#[handler(CreateWorkflowRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        action_config: Some(Default::default()),
        action_type: Some(Default::default()),
        condition_field: "example".to_string(),
        condition_operator: Default::default(),
        condition_value: Some("example".to_string()),
        condition_value_type: Some(Default::default()),
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        sequence: 42,
        workflow_id: "example".to_string(),
    })
}
