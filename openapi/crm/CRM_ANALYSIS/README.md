# CRM Competitive Analysis

> **Date:** 2026-05-10
> **Purpose:** Pitch-level competitive analysis for buyer decision-making
> **Scope:** RERP CRM vs. Odoo CRM, Salesforce, Microsoft Dynamics 365, SAP CRM, HubSpot, Pipedrive, Zoho CRM

---

## Overview

This analysis examines CRM capabilities across **12 functional components**, comparing RERP CRM against the competitive landscape from a buyer's perspective. Each component is documented as a pitch — the question a buyer asks and the answer their options provide.

The competitors evaluated:

| Vendor | Market Position | Best For | Pricing Model |
|--------|----------------|----------|---------------|
| **Salesforce** | Enterprise #1 | Large orgs, full ecosystem | Per-user/month ($25–$300+) |
| **Microsoft Dynamics 365** | Enterprise Contender | Microsoft-centric orgs | Per-user/month ($65–$200+) |
| **SAP CRM / C4C** | Enterprise B2B | Manufacturing, heavy ERP | Enterprise licensing |
| **HubSpot** | SMB Champion | SMB to mid-market | Free → Enterprise ($4,500+/mo) |
| **Pipedrive** | Sales-First | Small sales teams | Per-user/month ($15–$99) |
| **Zoho CRM** | Value Leader | Mid-market, budget-conscious | Per-user/month ($14–$52) |
| **Odoo CRM** | Open-Source CRM | DIY organizations | Free Community / Enterprise per-user |
| **RERP CRM** | Open-Source, API-First | Dev-driven orgs, data sovereignty | Self-hosted (free) / Hosted (TBD) |

---

## Component Directory

| # | Component | Directory | Status |
|---|-----------|-----------|--------|
| 1 | Lead & Contact Management | [lead-contact-management/README.md](lead-contact-management/README.md) | Planned |
| 2 | Pipeline & Stage Management | [pipeline-stage-management/README.md](pipeline-stage-management/README.md) | Planned |
| 3 | Opportunity & Revenue Management | [opportunity-revenue-management/README.md](opportunity-revenue-management/README.md) | Planned |
| 4 | Predictive Analytics | [predictive-analytics/README.md](predictive-analytics/README.md) | Planned |
| 5 | Sales Team Management | [sales-team-management/README.md](sales-team-management/README.md) | Planned |
| 6 | Marketing Integration | [marketing-integration/README.md](marketing-integration/README.md) | Planned |
| 7 | Communication Hub | [communication-hub/README.md](communication-hub/README.md) | Planned |
| 8 | Intelligence & Enrichment | [intelligence-enrichment/README.md](intelligence-enrichment/README.md) | Planned |
| 9 | Reporting & BI | [reporting-bi/README.md](reporting-bi/README.md) | Planned |
| 10 | Workflow Automation | [workflow-automation/README.md](workflow-automation/README.md) | Planned |
| 11 | Customer Engagement | [customer-engagement/README.md](customer-engagement/README.md) | Planned |
| 12 | Platform & Extensibility | [platform-extensibility/README.md](platform-extensibility/README.md) | Planned |

---

## Head-to-Head Capability Summary

| Capability Area | RERP | Odoo | Salesforce | Microsoft | SAP | HubSpot | Pipedrive | Zoho |
|----------------|------|------|------------|-----------|-----|---------|-----------|------|
| Lead/Contact Mgmt | ●○○ | ●●● | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ |
| Pipeline/Stages | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●○ | ●●● | ●●○ |
| Revenue Mgmt | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●○ | ●○○ | ●●○ |
| Predictive Analytics | ●○○ | ●●● | ●●● | ●●○ | ●●○ | ●●○ | ●○○ | ●●○ |
| Sales Team Mgmt | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●○ | ●○○ | ●●○ |
| Marketing Integration | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●● | ●○○ | ●●○ |
| Communication Hub | ●○○ | ●●○ | ●●● | ●●● | ●●○ | ●●● | ●●○ | ●●○ |
| Intelligence/Enrich | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●○ | ●○○ | ●●○ |
| Reporting & BI | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ |
| Workflow Automation | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●● | ●○○ | ●●○ |
| Customer Engagement | ●○○ | ●●○ | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●●○ |
| Platform/Extensibility | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●● | ●○○ | ●●○ |

**Legend:** ●●● = Full feature parity, ●●○ = Partial coverage, ●○○ = Planned / not yet implemented

---

## RERP CRM's Strategic Position

### Strengths
1. **OpenAPI-first architecture** — Every entity, endpoint, and schema is machine-readable. Enables automatic SDK generation, API contracts, and tooling. No other CRM exposes its data model this cleanly.
2. **Rust-based performance** — Axum + async I/O delivers sub-millisecond API latency. Bulk operations on 100,000+ records complete in seconds.
3. **Self-hosted, no vendor lock-in** — No per-seat pricing, no rate limits, no data egress fees. Full control over infrastructure and data.
4. **Two-crate codegen model** — Separation of generated (from OpenAPI) and implementation (business logic) enables safe regeneration.
5. **Modular service architecture** — The 7-service design (core, automation, livechat, engagement, intelligence, reporting, marketing) allows parallel development and independent scaling.

### Weaknesses (Current)
1. **Empty schema definitions** — The most critical gap. All sub-specs reference entities but schemas are blank.
2. **No entity relationships** — Leads, contacts, and opportunities are independent resources with no foreign keys.
3. **No pipeline model** — No stages, no probability, no won/lost semantics.
4. **No financial data model** — No revenue fields, no forecasting, no recurring revenue.
5. **No reporting surface** — No dashboards, pivot tables, or KPI endpoints.
6. **No enrichment integrations** — No Clearbit, ZoomInfo, or Hunter API connections.
7. **No workflow builder** — No visual automation surface for non-technical users.

### Threats
- **HubSpot's ecosystem lock-in** — Once contacts, deals, and tickets are built in HubSpot, migration cost is prohibitive.
- **Salesforce's AI moat** — Einstein adds features that RERP cannot match without ML engineering investment.
- **Microsoft's bundled advantage** — Dynamics 365 is free for Office 365 businesses. The friction to adopt is near zero.

### Opportunities
- **SMB/mid-market cost sensitivity** — Organizations tired of Salesforce's $200+/user/month pricing.
- **Developer-first organizations** — Teams that value API contracts over drag-and-drop builders.
- **Regulated industries** — Healthcare, finance, government — where self-hosting and data sovereignty are required.
- **AI-native CRM** — RERP's Rust infrastructure is ideal for embedding ML inference at API scale.

---

## Implementation Priority Matrix

| Priority | Component | Effort | Impact | Rationale |
|----------|-----------|--------|--------|-----------|
| **P0** | Lead/Contact Schemas | Low | Critical | Foundation — nothing works without complete schemas |
| **P0** | Pipeline & Stages | Medium | Critical | Core CRM feature — no pipeline = no CRM |
| **P1** | Opportunity/Revenue | Medium | High | Financial model is essential for sales teams |
| **P1** | Sales Team Management | Medium | High | Organization requires teams, not solo users |
| **P2** | Predictive Analytics | Medium | High | Differentiator — explainable AI is unique |
| **P2** | Reporting & BI | High | High | Management needs dashboards to justify CRM cost |
| **P3** | Marketing Integration | High | Medium | Important but can be phased; API form capture first |
| **P3** | Communication Hub | Medium | Medium | Email/SMS are table stakes; build after core |
| **P3** | Workflow Automation | High | Medium | Valuable but non-blocking for initial adoption |
| **P4** | Intelligence & Enrich | High | Medium | Nice-to-have; can integrate later |
| **P4** | Customer Engagement | High | Medium | Post-sale features come after sales CRM is solid |
| **P4** | Platform/Extensibility | High | Medium | Important for enterprise but not for first buyers |

---

## Quick Links

- [CRM Audit & Gap Analysis](../../docs/crm_audit_analysis.md) — Full technical gap analysis
- [Current OpenAPI Spec](../openapi.yaml) — RERP CRM gateway specification
- [Core Service Spec](../core/openapi.yaml) — Core entities sub-spec
- [Automation Service Spec](../automation/openapi.yaml) — Workflow/service automation spec
- [Livechat Service Spec](../livechat/openapi.yaml) — Livechat/engagement sub-spec
