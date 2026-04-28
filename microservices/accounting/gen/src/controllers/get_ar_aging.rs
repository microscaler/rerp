
// User-owned controller for handler 'get_ar_aging'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_ar_aging::{ Request, Response };



#[handler(GetArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "aging_date": "2024-01-31",
        //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //   "created_at": "2024-01-31T10:00:00Z",
        //   "currency_code": "USD",
        //   "current": 5500.0,
        //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
        //   "days_31_60": 0.0,
        //   "days_61_90": 0.0,
        //   "days_91_120": 0.0,
        //   "id": "a00a0e8400-e29b-41d4-a716-446655440000",
        //   "over_120": 0.0,
        //   "total_outstanding": 5500.0
        // }
    match serde_json::from_str::<Response>(r###"{
  "aging_date": "2024-01-31",
  "company_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2024-01-31T10:00:00Z",
  "currency_code": "USD",
  "current": 5500.0,
  "customer_id": "111e8400-e29b-41d4-a716-446655440001",
  "days_31_60": 0.0,
  "days_61_90": 0.0,
  "days_91_120": 0.0,
  "id": "a00a0e8400-e29b-41d4-a716-446655440000",
  "over_120": 0.0,
  "total_outstanding": 5500.0
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        
    }
    
    
}
