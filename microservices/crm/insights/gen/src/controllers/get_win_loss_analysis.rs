// User-owned controller for handler 'get_win_loss_analysis'.

use crate::handlers::get_win_loss_analysis::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetWinLossAnalysisController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "lost_count": 3,
    //   "lost_reasons": [
    //     {}
    //   ],
    //   "period": "Period",
    //   "revenue_won": "1250.00",
    //   "total_closed": 3,
    //   "win_rate": 12.5,
    //   "won_count": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "lost_count": 3,
  "lost_reasons": [
    {}
  ],
  "period": "Period",
  "revenue_won": "1250.00",
  "total_closed": 3,
  "win_rate": 12.5,
  "won_count": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        lost_count: 3,
        lost_reasons: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        period: "Period".to_string(),
        revenue_won: "1250.00".to_string(),
        total_closed: 3,
        win_rate: 12.5,
        won_count: 3,
    })
}
