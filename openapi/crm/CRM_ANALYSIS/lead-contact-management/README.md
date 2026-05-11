# Lead & Contact Management

> **Component:** Core CRM entity lifecycle
> **Competitive Landscape:** Salesforces, HubSpot, SAP, Microsoft Dynamics, Zoho, Pipedrive

## Pitch

**The Question Every Buyer Asks:** *"Can I capture, unify, and govern every person and organization that touches my business?"*

If the answer is no, you don't have a CRM — you have a spreadsheet with extra steps.

This component covers the foundational layer: how leads are captured, how contacts and accounts are managed, how records are enriched, deduplicated, and governed. It's the data model that everything else sits on top of.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM | Pipedrive |
|---------|----------|----------|------------|------------------------|---------|---------|----------|-----------|
| Lead entity | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Contact entity | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Account/Company entity | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lead-to-Contact conversion | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lead merge/dedup | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Contact enrichment | Planned | ✅ (IAP) | ✅ (Clearbit, ZoomInfo) | ✅ | ✅ (C4C) | ✅ (native) | ✅ (Lead Synthesis) | ✅ (Clearbit) |
| Lead mining/discovery | Planned | ✅ (IAP Mine) | ✅ (Lead Finder) | ✅ (LinkedIn) | ✅ | ✅ (Lead Intelligence) | ✅ | ✅ |
| Duplicate detection | Planned | ✅ | ✅ (Duplicate Rules) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Partner/reseller assign | Planned | ✅ | ✅ (Partner Portal) | ✅ | ✅ | ✅ | ✅ | ✅ |
| GDPR blacklisting | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| GDPR right-to-be-forgotten | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Multi-company support | Planned | ✅ | ✅ | ✅ | ✅ | ✅ (Enterprise) | ✅ | ✅ |
| Multi-language support | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Custom fields per entity | Planned | ✅ | ✅ (10K+ per object) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Record types/personas | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Activity timeline per record | Planned | ✅ | ✅ (Activity Timeline) | ✅ (Timeline) | ✅ | ✅ | ✅ | ✅ |
| File/attachment management | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Bulk import/export | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-first, machine-readable data model** — Every entity, relationship, and field is defined in OpenAPI specs, enabling automatic client SDK generation, API contracts, and tooling. No other CRM exposes its data model this cleanly.
- **Rust-based performance** — The underlying service architecture (Axum, async) offers sub-millisecond API latency and vertical scaling that interpreted platforms (Python, .NET CLR, Node.js) cannot match at equivalent hardware.
- **Self-hosted, no vendor lock-in** — Unlike every competitor except Odoo Community, RERP runs entirely under your control. No per-seat pricing, no rate limits, no data egress fees.

### Where RERP Lags
- **Schema definitions are empty** — The current sub-specs reference entities but schemas are blank. This is the immediate first deliverable.
- **No enrichment integrations** — Lead enrichment from Clearbit, ZoomInfo, Hunter, or equivalent requires API integrations that take engineering cycles.
- **No web form capture** — HubSpot and Salesforce let you embed a form on any website. RERP has zero web capture surface today.
- **No record types/personas** — Salesforce's Account Record Types (Customer vs Partner vs Prospect) and field-level page layouts are standard enterprise expectations.
- **No duplicate detection algorithm** — Odoo has `duplicate_lead_ids`/`duplicate_lead_count`. Salesforce has Duplicate Rules. RERP has nothing.

---

## Competitive Intelligence Deep Dive

### Salesforce Lightning (Enterprise Leader — ~$25–$330/user/month)
Salesforce's lead/contact model is the gold standard for enterprise. **Account-Contact-Opportunity** is the holy trinity. Contact Role on Opportunity, multiple phone/email fields, industry classification, billing/shipping addresses, partner roles, and a unified **Activity Timeline**. **Duplicate Rules** prevent entries before they're created. Custom objects extend to 10,000+ fields per object. The differentiator is **Einstein Data Cloud** — real-time identity resolution across channels, plus **Lead Scoring** (1–100) that weighs 200+ signals. **Partner Portal** enables reseller self-service. Enterprise tier includes **Data Cloud** with cross-channel merge-and-govern.

### Microsoft Dynamics 365 Sales (Enterprise Contender — ~$65–$200/user/month)
Dynamics offers **Contact Intelligence** (LinkedIn data enrichment via native integration), AI-powered lead scoring, and deep integration with Teams. **Customer Insights** (formerly D365 CDP) provides unified customer profiles from 360+ data sources. Email and calendar integration from Outlook is unmatched — every sent email is logged automatically. **Sales Hub** includes lead capture from LinkedIn, web forms, and social media. For Microsoft-centric enterprises, the friction to adopt is near zero because it ships with Office 365 E5 licenses.

### SAP CRM / SAP S/4HANA Cloud (Enterprise B2B — custom pricing)
SAP's CRM is deeply integrated with SAP ERP — order management, pricing, inventory, and finance data are first-class citizens. The lead-contact-account model extends directly into **S/4HANA Business Partner** (a unified entity covering customers, suppliers, and partners in one model). Best for manufacturing and heavy B2B where CRM data flows directly into production planning, inventory, and order fulfillment. **SAP C4C** (Customer Cloud) is the cloud-native offering with mobile-first agent apps.

### HubSpot CRM (SMB Champion — Free → $1,800+/month for Enterprise)
HubSpot's lead management is frictionless. **Free CRM** captures contacts, deals, and notes with zero cost. **Contact Properties** support 2,000+ custom properties per contact. **Company Properties** automatically enrich from Clearbit data on create. **Lead Intelligence** turns anonymous website visitors into identified contacts using Clearbit. **CRM Import** supports CSV, Excel, and migration from 100+ tools. **Contact merging** detects and combines duplicate records automatically. The flywheel: free → paid → sticky. Once 288,000+ customers in 135+ countries have data in, migration cost is prohibitive.

### Zoho CRM (Value Leader — $14–$52/user/month)
Zoho offers **Lead Synthesis** (auto-enrichment from Zoho's own Verto data platform), **Lead Scoring** powered by Zia AI, multi-level approval workflows, and integration with 50+ Zoho apps. **Canvas** lets users redesign the CRM layout without code. **Client Scripts** enable custom JavaScript logic. **Business Cards** capture contact data from physical business cards via OCR. **Lead Portals** allow prospects to self-update their information. Best value proposition with an ecosystem of 40+ integrated apps (Mail, Campaigns, Desk, Bookings).

### Pipedrive (Sales-First — $15–$99/user/month)
Pipedrive strips CRM down to the bare minimum: contacts, companies, deals, and activities. No accounts, no opportunities — **Deals** are the pipeline unit. No enrichment, no web forms. The pitch is "sales teams don't need a CRM, they need a pipeline tool." **Activity-Based Selling** enforces that every deal stage requires a logged activity before moving forward. **500+ integrations** including Google Maps, Google Workspace, Zapier, and Slack. For simple sales teams under 20, this is the right answer. Beyond that, enterprises need more.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2-4 weeks)
1. Define complete OpenAPI schemas for Lead, Contact, and Opportunity entities
2. Establish entity relationships: Lead → Contact → Account
3. Implement basic deduplication via email address matching
4. Add contact fields: email, phone, address, company, function
5. Add account fields: name, industry, size, website

### Phase 2 (Short-term — 4-8 weeks)
1. Implement lead-to-contact conversion endpoint
2. Implement lead merge endpoint (with conflict resolution)
3. Add custom fields schema (key-value store per entity)
4. Add GDPR fields: blacklisted, consent, data_retention_until
5. Add bulk import via CSV endpoint

### Phase 3 (Medium-term — 8-12 weeks)
1. Lead enrichment via third-party API (Clearbit or equivalent)
2. Web form capture endpoint (public, rate-limited)
3. Partner/reseller assignment rules
4. Record types with field-level access per type
5. Activity timeline per record

---

## Key Takeaway for Buyers

RERP CRM's pitch is **open, fast, and self-hosted**. If your organization values data sovereignty, API-first design, and performance over out-of-the-box feature richness, RERP is the compelling choice. The gap with Salesforce/HubSpot is feature depth today — but RERP's OpenAPI-first architecture means the feature gap closes faster than proprietary platforms where you're at the mercy of vendor release cycles.

The immediate priority: **fill the schema definitions**. Without complete schemas, nothing else matters.
