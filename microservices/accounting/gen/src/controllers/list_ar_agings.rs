
// User-owned controller for handler 'list_ar_agings'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_ar_agings::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::GeneralLedgerAccountsReceivableArAging;



#[handler(ListArAgingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "items": [
        //     {
        //       "aging_date": "2024-01-31",
        //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //       "created_at": "2024-01-31T10:00:00Z",
        //       "currency_code": "USD",
        //       "current": 5500.0,
        //       "customer_id": "111e8400-e29b-41d4-a716-446655440001",
        //       "days_31_60": 0.0,
        //       "days_61_90": 0.0,
        //       "days_91_120": 0.0,
        //       "id": "a00a0e8400-e29b-41d4-a716-446655440000",
        //       "over_120": 0.0,
        //       "total_outstanding": 5500.0
        //     }
        //   ],
        //   "limit": 20,
        //   "page": 1,
        //   "total": 1
        // }
    match serde_json::from_str::<Response>(r###"{
  "items": [
    {
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
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 1
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        items: Some(vec![serde_json::from_value::<GeneralLedgerAccountsReceivableArAging>(serde_json::json!({"aging_date":"2024-01-31","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-31T10:00:00Z","currency_code":"USD","current":5500.0,"customer_id":"111e8400-e29b-41d4-a716-446655440001","days_31_60":0.0,"days_61_90":0.0,"days_91_120":0.0,"id":"a00a0e8400-e29b-41d4-a716-446655440000","over_120":0.0,"total_outstanding":5500.0})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
    
    
}
