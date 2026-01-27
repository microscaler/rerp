// User-owned controller for handler 'update_asset_register'.
use crate::handlers::update_asset_register::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAssetRegisterController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "code": "REAL_ESTATE",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "id": "a0130e8400-e29b-41d4-a716-446655440000",
    //   "is_active": true,
    //   "name": "Real Estate Register (Updated)",
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        code: "REAL_ESTATE".to_string(),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        default_depreciation_method: Some("example".to_string()),
        default_useful_life_months: Some(42),
        description: Some("example".to_string()),
        id: "a0130e8400-e29b-41d4-a716-446655440000".to_string(),
        is_active: true,
        name: "Real Estate Register (Updated)".to_string(),
        parent_id: Some("example".to_string()),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
    }
}
