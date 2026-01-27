// User-owned controller for handler 'list_asset_registers'.
use crate::handlers::list_asset_registers::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AssetRegister;

#[handler(ListAssetRegistersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "code": "REAL_ESTATE",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "id": "a0130e8400-e29b-41d4-a716-446655440000",
    //       "is_active": true,
    //       "name": "Real Estate Register",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<AssetRegister>(serde_json::json!({"code":"REAL_ESTATE","created_at":"2024-01-15T10:00:00Z","id":"a0130e8400-e29b-41d4-a716-446655440000","is_active":true,"name":"Real Estate Register","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
