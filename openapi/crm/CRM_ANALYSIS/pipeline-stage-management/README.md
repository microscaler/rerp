# Pipeline & Stage Management

> **Component:** Kanban pipeline with configurable stages, flow control, and probability mapping
> **Priority:** P0 — The daily dashboard every sales rep and manager lives on
> **Odoo Reference:** crm.stage (67 lines), crm.lead.stage behavior, rotting_threshold_days

---

## The Pitch

**Buyer Question:** *Can I visualize my entire sales process from first touch to closed deal, with stages that match my actual business process — not the other way around?*

Pipeline management is the single most-used CRM feature. Every sales rep opens their CRM to see the Kanban board. Every manager checks it for pipeline health. If your CRM forces you into a pipeline shape that doesn't match your sales process, you'll hack around it. Eventually, you'll abandon it. This component defines the stages, the probability mapping, the transition rules, and the flow control that makes a pipeline a pipeline rather than a flat list.

---

## What This Component Does

Pipeline & Stage Management is the operational heart of the CRM. It handles:

1. **Stage Configuration** — Define the stages that match your sales process (New, Qualified, Proposal, Negotiation, Won, Lost)
2. **Probability Mapping** — Each stage maps to a win probability (New: 10%, Qualified: 25%, Proposal: 50%, Won: 100%)
3. **Flow Control** — Rules that control which transitions are allowed (e.g., can't skip from New to Won)
4. **Rotting Detection** — Alerts when deals have been sitting in a stage too long
5. **Won/Lost Semantics** — When a deal reaches the final stage, probability = 100%, date_closed is set
6. **Lost Reason Tracking** — Why deals are lost (Price, Feature, Competitor, Timing, No Decision)
7. **Stage History** — Audit trail of when and how each deal moved through stages
8. **Pipeline Summary** — Real-time count and revenue at each stage

---

## Entity Model

### Stage Entity

The core entity that defines the pipeline shape:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Stage name (e.g., "New", "Qualified", "Proposal", "Negotiation", "Won") |
| `sequence` | Integer | Yes | Order position in pipeline (lower = earlier); used for Kanban left-to-right ordering |
| `probability` | Integer (0-100) | Yes | Win probability when in this stage; auto-set on opportunity expected_revenue = amount * probability |
| `is_won` | Boolean | No | Marks this as the final "Won" stage; when set to true, auto-sets probability=100, date_closed=now |
| `is_lost` | Boolean | No | Marks this as a "Lost" stage |
| `rotting_threshold_days` | Integer | No | Days before stage shows as stale in Kanban; visual highlighting |
| `requirements` | Text | No | Internal requirements/tooltip for this stage (e.g., "Must have signed NDA") |
| `team_ids` | Many2Many: crm.team | No | Team association; stages can be team-specific |
| `fold` | Boolean | No | Folded in Kanban when no deals are in this stage |
| `color` | Integer | No | Kanban color (1-16); visual identification |
| `description` | Text | No | Stage description for user documentation |
| `sequence` | Integer | Yes | Order in pipeline (1, 2, 3, ...) |

### Default Stage Seed Data

Every CRM ships with sensible defaults:

| Sequence | Stage Name | Probability | Is Won | Color |
|----------|-----------|-------------|--------|-------|
| 1 | New | 0 | false | 11 (blue) |
| 2 | Qualified | 20 | false | 5 (green) |
| 3 | Proposition | 40 | false | 8 (orange) |
| 4 | Negotiation | 60 | false | 3 (red) |
| 5 | Won | 100 | true | 10 (green-dark) |

Teams can customize this: add stages, remove stages, change probabilities.

### Stage History Entity

Tracks every stage transition:

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `lead_id` | Foreign Key: Lead | The lead/opportunity |
| `old_stage_id` | Foreign Key: Stage | Previous stage (null if first) |
| `new_stage_id` | Foreign Key: Stage | New stage |
| `probability_before` | Integer (0-100) | Probability before transition |
| `probability_after` | Integer (0-100) | Probability after transition |
| `user_id` | Foreign Key: User | Who made the transition |
| `transition_date` | DateTime | When transition occurred |
| `days_in_old_stage` | Float | How long in old stage before transition |

### Lost Reason Entity

Why deals are lost:

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `name` | String (128) | Reason name (e.g., "Price", "Feature Gap", "Competitor", "Timing", "No Decision") |
| `category` | Enum: [PRICE, FEATURE, COMPETITOR, TIMING, STRATEGIC, OTHER] | Classification |
| `sequence` | Integer | Display order |
| `active` | Boolean | Soft delete |

### Lost Reason Assignment on Lead

| Field | Type | Purpose |
|-------|------|---------|
| `lost_reason_id` | Foreign Key: LostReason | Why this deal was lost |
| `won_status` | Enum: [PENDING, WON, LOST] | Derived from stage's is_won/is_lost flags |

---

## Entity Relationships

```
crm.lead → crm.stage (via stage_id)
crm.lead → crm.lost.reason (via lost_reason_id, when won_status = LOST)
crm.lead_stage_history (audit trail)
  ├── crm.lead (via lead_id)
  ├── crm.stage → old_stage_id
  └── crm.stage → new_stage_id

crm.stage → crm.team (Many2Many via team_ids)

crm.lead.expected_revenue × crm.stage.probability / 100 = crm.lead.prorated_revenue
```

---

## Stage Transition Logic

When a lead's stage changes, the following computations happen:

1. **Probability Update** — `probability = stage.probability` (auto-set from stage)
2. **Revenue Recompute** — `prorated_revenue = expected_revenue * probability / 100`
3. **Timestamp** — `date_last_stage_update = now()`
4. **History Record** — Create stage_history entry with old/new stage, probabilities, days_in_old_stage
5. **Won/Lost Detection**:
   - If `stage.is_won`: set `won_status = WON`, `date_closed = now()`, `probability = 100`
   - If `stage.is_lost`: set `won_status = LOST`
6. **Rotting Check** — If `now() - date_last_stage_update > stage.rotting_threshold_days`, highlight in Kanban

### Transition Validation

Before allowing a stage change:

1. **Sequence Forward** — By default, stages can only move forward (sequence: new > current). Backward moves require admin override.
2. **Stage Requirements** — If `stage.requirements` is defined, validate that required conditions are met (e.g., "has email", "has phone", "has meeting logged").
3. **Lost Reason Required** — If transitioning to a `is_lost` stage, `lost_reason_id` must be set.
4. **Won Confirmation** — If transitioning to `is_won` stage, `expected_revenue` and `date_deadline` must be populated.

---

## Required API Endpoints

### Stage CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/stages` | List all stages (ordered by sequence) |
| `GET` | `/stages/{id}` | Get stage detail |
| `POST` | `/stages` | Create new stage |
| `PATCH` | `/stages/{id}` | Update stage (name, probability, rotting) |
| `DELETE` | `/stages/{id}` | Delete stage (only if no leads in it) |
| `PUT` | `/stages/reorder` | Reorder stages by new sequence |

### Stage Transitions

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PATCH` | `/leads/{id}/stage` | Move lead to new stage |
| `POST` | `/leads/{id}/transition` | Transition with validation, lost reason, comments |
| `GET` | `/leads/{id}/stage-history` | Full stage transition history |

### Pipeline Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/pipeline/summary` | Count and revenue per stage (Kanban data) |
| `GET` | `/pipeline/weighted` | Total weighted pipeline (Σ amount × probability) |
| `GET` | `/pipeline/forecast` | Forecast by close date period |
| `GET` | `/stages/{id}/rotting` | Leads stuck in stage > rotting_threshold_days |

### Lost Reason Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/lost-reasons` | List all lost reasons |
| `POST` | `/lost-reasons` | Create lost reason |
| `PATCH` | `/lost-reasons/{id}` | Update lost reason |

---

## Odoo Technical Patterns to Follow

### Pattern 1: is_won Triggers on Stage Write
When `is_won` changes on a stage, Odoo automatically:
- Sets all leads in that stage to `probability = 100`
- Sets `won_status = 'won'`
- Sets `date_closed = now()`
- If `is_won` is removed, recomputes automated probability

**Recommendation: RERP should implement this as a cascade trigger on the Stage entity.**

### Pattern 2: Rotting Threshold for Stale Deals
Odoo's `rotting_threshold_days` is used for visual Kanban highlighting (deals past threshold get a red border). This is not an automated action — it's a display concern.

**Recommendation: RERP should compute rotting status on the API level and return a `is_rotting` flag per lead. The UI renders the visual; the API computes the data.**

### Pattern 3: Stage Sequence is the Source of Truth
The `sequence` field on Stage determines both the Kanban order AND the transition rules (can only move forward in sequence). There is no separate "transition matrix."

**Recommendation: RERP should use sequence as the single source of truth for order and allowed transitions. This eliminates a whole class of bugs.**

### Pattern 4: Probability on Stage Drives Revenue Calculation
`prorated_revenue = expected_revenue × probability / 100` is computed automatically when the stage changes. This means revenue forecasts are always up-to-date with pipeline position.

**Recommendation: RERP's prorated_revenue must be a computed field, never manually edited.**

---

## Competitive Positioning

### Where RERP Wins
- **API-defined pipelines** — Pipeline structure is defined in OpenAPI specs, not UI configuration. Every client (web, mobile, CLI) gets the same pipeline definition automatically.
- **Rust-level stage transition speed** — When a lead changes stage, all downstream calculations (probability, revenue, scoring) recompute in Rust. No ORM overhead.
- **Self-hosted stage customization** — No vendor limitations on how many stages or what rules you can define.

### Where RERP Lags
- **No Stage entity at all** — The spec has Leads and Opportunities but no Stages entity. No probability mapping, no won/lost flags, no rotting thresholds.
- **No stage history** — Without tracking transitions, pipeline analytics are impossible.
- **No multi-pipeline support** — No way to have different pipeline shapes for different teams.

---

## Competitive Intelligence Deep Dive

### Salesforce: Sales Path with Required Fields
Salesforce's **Sales Path** provides stage-specific guidance with custom questions and fields for each stage. **Path Assistant** highlights required fields at each stage. **Record Types** enable different pipelines for different products or business units. **Pipeline Forecasts** roll up opportunities by stage with categories (O/P/Closed/Worst/Best). **Einstein Opportunity Scoring** predicts win probability per deal using ML.

### Pipedrive: Activity-Based Selling
Pipedrive's pipeline IS the product. Every stage requires a logged activity before moving forward. This prevents "pipeline hoarding" — deals sitting in late stages without engagement. Revenue widget shows total weighted pipeline. Activities dashboard tracks calls, emails, meetings per rep. For teams under 20 who want simplicity: unmatched. For managers who need rich analytics: insufficient.

### HubSpot: Pipeline Switching
HubSpot lets you have multiple pipelines (Sales, Marketing, Customer Success) and switch between them without confusion. **Stalled Deals** auto-flag deals that haven't progressed in a set time, with automated follow-up tasks. **Deal Pipelines** show revenue at each stage in real-time. Simple but effective.

### Zoho: Blueprint Enforcement
Zoho's **Blueprint** enforces strict stage-by-stage workflows — deals can't skip stages, and each stage requires specific data entry or approvals. **Sales Signals** AI alerts when deals look at-risk. **Territory Management** assigns deals based on geographic, product, or industry rules. Best value for mid-market.

---

## Implementation Roadmap

### Phase 1: Core Stage Model (1-2 weeks) — P0
1. Define `Stage` entity: id, name, sequence, probability, is_won, is_lost, team_ids, rotting_threshold_days, requirements, fold, color
2. Add `stage_id` Foreign Key to Lead entity
3. Implement probability auto-assignment on stage change
4. Compute `prorated_revenue = expected_revenue × probability / 100`
5. Seed default stages (New, Qualified, Proposition, Won)

### Phase 2: History & Won/Lost (2-3 weeks) — P0
1. Define `StageHistory` entity with full transition audit trail
2. Define `LostReason` entity with reason codes and categories
3. Add `lost_reason_id` and `won_status` to Lead
4. Implement won detection cascade (is_won → probability=100, date_closed=now)
5. Implement stage change trigger that creates history record

### Phase 3: Rotting & Validation (2-3 weeks) — P1
1. Implement rotting detection (`now() - date_last_stage_update > threshold`)
2. Add `is_rotting` flag computed on read
3. Implement stage transition validation (sequence forward only, requirements check)
4. Add lost reason required on loss transitions
5. Pipeline summary endpoint (count/revenue/weighted per stage)

### Phase 4: Multi-Pipeline (3-4 weeks) — P1
1. Add `team_id` to Stage for team-specific pipelines
2. Implement multi-pipeline selection (user chooses pipeline per view)
3. Stage change audit log with full history
4. Stage SLA monitoring (time in stage vs expected duration)
5. Pipeline comparison across time periods

---

## Key Takeaway for Buyers

Pipeline management is the make-or-break feature. A buyer will evaluate your CRM on one question: *Can I set up my actual sales process, or do I have to change my process to fit your CRM?* RERP's answer through OpenAPI-first design means once the Stage entity is defined, every client gets the pipeline automatically — that's the architectural advantage. But the work must be done fast: no pipeline = no CRM, period.
