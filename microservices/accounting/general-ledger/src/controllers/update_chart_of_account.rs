// User-owned controller for handler 'update_chart_of_account'.
use crate::handlers::update_chart_of_account::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "account_type": "ASSET",
    //   "code": "1",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "description": "Updated description",
    //   "id": "a00c0e8400-e29b-41d4-a716-446655440000",
    //   "is_active": true,
    //   "level": 0,
    //   "name": "Assets (Updated)",
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        account_type: "ASSET".to_string(),
        code: "1".to_string(),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        description: Some("Updated description".to_string()),
        id: "a00c0e8400-e29b-41d4-a716-446655440000".to_string(),
        is_active: true,
        level: 0,
        name: "Assets (Updated)".to_string(),
        parent_id: Some("example".to_string()),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
    }
}
