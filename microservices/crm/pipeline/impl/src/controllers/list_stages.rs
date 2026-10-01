// BRRTRouter: user-owned
//
// The Kanban columns: every stage, or one pipeline's with filter_team_id
// (Sales or Recruitment - see supabase::TEAM_*). `fold` marks the closed-out
// lane the board collapses.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::list_stages::Request;
use serde_json::{json, Value};

#[handler(ListStagesController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_viewer(req.jwt_claims.as_ref()) {
        return denied;
    }
    let team = req.data.filter_team_id.as_deref().filter(|t| !t.is_empty());
    let items: Vec<Value> = crate::supabase::stages_for(team)
        .map(|s| {
            json!({
                "id": s.id,
                "name": s.name,
                "sequence": s.sequence,
                "probability": s.probability,
                "is_won": s.is_won,
                "is_lost": s.is_lost,
                "fold": s.fold,
                "color": s.color,
                "requirements": s.requirements,
                "team_ids": [s.team],
            })
        })
        .collect();
    let total = items.len() as i32;
    HttpJson::new(
        200,
        json!({ "items": items, "total": total, "page": 1, "limit": total.max(1) }),
    )
}
