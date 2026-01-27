// User-owned controller for handler 'get_asset'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::get_asset::{Request, Response};

#[handler(GetAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "asset_number": "AST-2024-001",
    //   "category_id": "a0110e8400-e29b-41d4-a716-446655440000",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "currency_code": "USD",
    //   "current_value": 5500000.0,
    //   "depreciation_method": "STRAIGHT_LINE",
    //   "id": "a0100e8400-e29b-41d4-a716-446655440000",
    //   "name": "Office Building - Main Campus",
    //   "purchase_cost": 5000000.0,
    //   "purchase_date": "2020-01-15",
    //   "status": "ACTIVE",
    //   "updated_at": "2024-01-15T10:00:00Z",
    //   "useful_life_months": 360
    // }

    Response {
        accumulated_depreciation: Some(3.14),
        accumulated_depreciation_account_id: Some("example".to_string()),
        asset_account_id: Some("example".to_string()),
        asset_number: "AST-2024-001".to_string(),
        category_id: "a0110e8400-e29b-41d4-a716-446655440000".to_string(),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        current_value: Some(5500000.0),
        depreciation_expense_account_id: Some("example".to_string()),
        depreciation_method: Some("STRAIGHT_LINE".to_string()),
        depreciation_rate: Some(3.14),
        description: Some("example".to_string()),
        disposal_date: Some("example".to_string()),
        id: "a0100e8400-e29b-41d4-a716-446655440000".to_string(),
        in_service_date: Some("example".to_string()),
        location: Some("example".to_string()),
        metadata: Some(Default::default()),
        name: "Office Building - Main Campus".to_string(),
        purchase_cost: Some(5000000.0),
        purchase_date: "2020-01-15".to_string(),
        salvage_value: Some(3.14),
        status: "ACTIVE".to_string(),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
        useful_life_months: Some(360),
    }
}
