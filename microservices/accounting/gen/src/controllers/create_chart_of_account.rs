
// User-owned controller for handler 'create_chart_of_account'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_chart_of_account::{ Request, Response };



#[handler(CreateChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "account_type": "ASSET",
        //   "code": "1",
        //   "created_at": "2024-01-15T10:00:00Z",
        //   "description": "Root asset account",
        //   "id": "a00c0e8400-e29b-41d4-a716-446655440000",
        //   "is_active": true,
        //   "level": 0,
        //   "name": "Assets",
        //   "updated_at": "2024-01-15T10:00:00Z"
        // }
    match serde_json::from_str::<Response>(r###"{
  "account_type": "ASSET",
  "code": "1",
  "created_at": "2024-01-15T10:00:00Z",
  "description": "Root asset account",
  "id": "a00c0e8400-e29b-41d4-a716-446655440000",
  "is_active": true,
  "level": 0,
  "name": "Assets",
  "updated_at": "2024-01-15T10:00:00Z"
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        account_type: "ASSET".to_string(),code: "1".to_string(),created_at: Some("2024-01-15T10:00:00Z".to_string()),description: Some("Root asset account".to_string()),id: "a00c0e8400-e29b-41d4-a716-446655440000".to_string(),is_active: true,level: 0,name: "Assets".to_string(),parent_id: Some("example".to_string()),updated_at: Some("2024-01-15T10:00:00Z".to_string()),
    }
    
    
}
