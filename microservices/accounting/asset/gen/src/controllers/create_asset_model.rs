// User-owned controller for handler 'create_asset_model'.

use crate::handlers::create_asset_model::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAssetModelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        asset_account_id: Some("example".to_string()),
        depreciation_account_id: Some("example".to_string()),
        depreciation_method: "example".to_string(),
        expense_account_id: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        residual_value_percent: Some(3.14),
        useful_life_months: 42,
    }
}
