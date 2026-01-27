// User-owned controller for handler 'list_assets'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::list_assets::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_bff_gen::handlers::types::Asset;

#[handler(ListAssetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "asset_number": "AST-2024-001",
    //       "category_id": "a0110e8400-e29b-41d4-a716-446655440000",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "current_value": 5500000.0,
    //       "depreciation_method": "STRAIGHT_LINE",
    //       "id": "a0100e8400-e29b-41d4-a716-446655440000",
    //       "name": "Office Building - Main Campus",
    //       "purchase_cost": 5000000.0,
    //       "purchase_date": "2020-01-15",
    //       "status": "ACTIVE",
    //       "updated_at": "2024-01-15T10:00:00Z",
    //       "useful_life_months": 360
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<Asset>(serde_json::json!({"asset_number":"AST-2024-001","category_id":"a0110e8400-e29b-41d4-a716-446655440000","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","current_value":5500000.0,"depreciation_method":"STRAIGHT_LINE","id":"a0100e8400-e29b-41d4-a716-446655440000","name":"Office Building - Main Campus","purchase_cost":5000000.0,"purchase_date":"2020-01-15","status":"ACTIVE","updated_at":"2024-01-15T10:00:00Z","useful_life_months":360})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
