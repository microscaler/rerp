# Pipeline & Stage Management

> **Component:** Kanban pipeline with configurable stages and flow control
> **Competitive Landscape:** Salesforce, HubSpot, Pipedrive, Microsoft Dynamics, SAP, Zoho

## Pitch

**The Question Every Buyer Asks:** *"Can I visualize my entire sales process from first touch to closed deal, with stages that match my actual business process?"*

If your CRM forces you into a pipeline shape that doesn't match your sales process, you'll hack around it. Eventually, you'll abandon it. Pipeline management is the single most-used CRM feature — it's the daily dashboard every sales rep and manager lives on.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM | Pipedrive |
|---------|----------|----------|------------|------------------------|---------|---------|----------|-----------|
| Configurable stages | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stage probability (%) | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stage type (Won/Lost/Open) | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stage rotting/slippage alerts | Planned | ✅ (rotting_threshold_days) | ✅ (Path Assistant) | ✅ (Sales Path) | ✅ | ✅ (Stalled Deals) | ✅ | ✅ |
| Stage requirements/validation | Planned | ✅ (requirements on stage) | ✅ (Page Layout validation) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Multi-pipeline per team | Planned | ❌ | ✅ (Opportunity Path) | ✅ (Process Flow) | ✅ | ✅ (Deals pipeline) | ✅ | ✅ |
| Stage drag-and-drop API | Planned | ✅ (view-level) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ (primary UX) |
| Stage history tracking | Planned | ✅ (date_last_stage_update) | ✅ (Field History) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stage change audit log | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Win/Lose tracking | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lost reason codes | Planned | ✅ | ✅ (Picklist) | ✅ | ✅ | ✅ (Deals > 30d) | ✅ | ✅ |
| Win reason codes | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Stage transition rules | Planned | ❌ | ✅ (Flow Builder) | ✅ (Power Automate) | ✅ | ✅ (Workflows) | ✅ (Workflows) | ❌ |
| Stage-specific email templates | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Stage SLA monitoring | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Pipeline stage grouping | Planned | ❌ | ✅ (Folders) | ✅ | ✅ | ✅ | ✅ | ❌ |

---

## Competitive Positioning

### Where RERP Wins
- **Rust-level stage transition speed** — When a lead changes stage, all downstream calculations (probability, revenue, scoring) recompute in Rust. No ORM overhead, no interpreted language lag.
- **API-defined pipelines** — Pipeline structure is defined in OpenAPI specs, not UI configuration. This means every client (web, mobile, CLI) gets the same pipeline definition automatically.
- **Self-hosted stage customization** — No vendor limitations on how many stages or what rules you can define.

### Where RERP Lags
- **No stage model at all** — The spec has `Leads` and `Opportunities` but no `Stages` entity. No probability mapping, no won/lost flags, no rotting thresholds. This is the highest-priority gap after schemas.
- **No stage history** — Without tracking when and how a lead moved through stages, pipeline analytics are impossible.
- **No multi-pipeline support** — Salesforce handles different pipeline shapes for different products, regions, and teams. RERP has a single flat pipeline implied by entity type.

---

## Competitive Intelligence Deep Dive

### Salesforce (Enterprise Pipeline Standard — $25–$330/user/month)
Salesforce's **Opportunity Object** is the gold standard. Fields: Amount, Expected Revenue (Amount × Probability), Probability (0–100%), Close Date, Stage, Type, Lead Source, Next Step, Campaign, and a complete **Stage Name** picklist with automated probability mapping. **Sales Path** (Sales Cloud Einstein) provides stage-specific guidance with custom questions and fields for each stage. **Record Types** enable different pipelines for different products or business units. **Pipeline Forecasts** roll up opportunities by stage with rollup summaries and forecast categories (O/P/Closed/Worst/Best). **Einstein Opportunity Scoring** predicts win probability per deal. Enterprise tier includes **Path Assistant** that highlights required fields at each stage.

### Microsoft Dynamics 365 (Process Automation — $65–$200/user/month)
Dynamics uses **Business Process Flows** — horizontal stage indicators across forms that enforce required fields at each stage. **Process Nodes** can mandate data entry before stage transitions. **Power Automate** triggers stage-change workflows (e.g., "when stage = Proposal, send contract template, notify legal"). **Sales Hub** includes deal velocity tracking (average days per stage). **Field Service** integration adds on-site visit stages. Best for Microsoft-centric organizations already using Power Platform for process automation.

### HubSpot (Simple Pipeline Champion — Free → $1,800+/month for Enterprise)
HubSpot's pipeline is deceptively simple: create stages, assign probabilities, done. The strength is **Pipeline Switching** — you can have multiple pipelines (Sales, Marketing, Customer Success) and switch them without confusion. **Stalled Deals** auto-flag deals that haven't progressed in a set time, with automated follow-up tasks assigned to the rep. **Deal Pipelines** show revenue at each stage in real-time. The pitch: "if you can't configure your pipeline in 5 minutes, HubSpot isn't for you." The tradeoff: limited stage-specific validation rules compared to Salesforce.

### Pipedrive (Activity-Driven Pipeline — $15–$99/user/month)
Pipedrive's pipeline is the entire product. No accounts, no opportunities — just **Deals** that flow through stages. The strength is **Activity-Based Selling**: every stage requires a logged activity (call, email, meeting) before moving to the next. This prevents "pipeline hoarding" — deals sitting in late stages without engagement. **Revenue** widget shows total weighted pipeline in real-time. **Activities** dashboard tracks calls, emails, and meetings per rep. For teams under 20 who want simplicity: unmatched. For managers who need rich pipeline analytics: insufficient.

### Zoho CRM (Value Pipeline — $14–$52/user/month)
Zoho's **Sales Stages** support multiple pipelines, stage colors, probability mapping, and **Sales Signals** (AI alerts when deals look at-risk based on inactivity, stage slippage, or unusual patterns). **Blueprint** enforces strict stage-by-stage workflows — deals can't skip stages, and each stage can require specific data entry or approvals. **Territory Management** assigns deals based on geographic, product, or industry rules. Best value for mid-market teams who need enterprise pipeline control without enterprise pricing.

### SAP CRM (Manufacturing/Heavy B2B — custom pricing)
SAP's pipeline integrates with **S/4HANA** order management, inventory, and pricing. Stages map to business milestones (e.g., "Quotation Approved" → "Production Scheduled" → "Shipping Confirmed"). **Customer History** shows the full interaction timeline across all touchpoints. Best for manufacturing and distribution where CRM pipeline directly drives production planning and inventory allocation.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 1-2 weeks)
1. Define `Stage` entity: id, name, sequence, probability, is_won, is_lost, team_ids, rotting_threshold_days, requirements
2. Add `stage_id` field to Lead and Opportunity entities
3. Implement probability auto-assignment based on stage
4. Add `date_last_stage_update` tracking to Lead/Opportunity
5. Implement stage change history tracking

### Phase 2 (2-4 weeks)
1. Implement `LostReason` entity with reason codes
2. Add `lost_reason_id` to Lead/Opportunity
3. Implement won/lost status computation
4. Add stage transition API (with validation)
5. Implement stage rotting detection (cron)

### Phase 3 (4-6 weeks)
1. Multi-pipeline support (per team)
2. Stage-specific validation rules
3. Stage change audit log
4. Pipeline summary endpoint (count/revenue per stage)
5. Stage SLA monitoring

---

## Key Takeaway for Buyers

Pipeline management is the make-or-break feature. A buyer will evaluate your CRM on one question: *"Can I set up my actual sales process, or do I have to change my process to fit your CRM?"* RERP's answer today is "we're building this" — which needs to become a concrete, working implementation fast. The OpenAPI-first approach means once the Stage entity is defined, every client gets the pipeline automatically. That's the architectural advantage.
