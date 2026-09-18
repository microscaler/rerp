// User-owned controller for handler 'create_asset_category'.

use crate::handlers::create_asset_category::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAssetCategoryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        default_depreciation_method: Some("example".to_string()),
        default_useful_life_months: Some(42),
        description: Some("example".to_string()),
        gl_account_credit: Some("example".to_string()),
        gl_account_debit: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        parent_category_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    })
}
