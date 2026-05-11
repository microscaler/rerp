# Intelligence & Enrichment

> **Component:** Automated data enrichment, lead discovery, and company intelligence
> **Competitive Landscape:** Salesforce Clearbit, Microsoft LinkedIn, HubSpot Lead Intelligence, ZoomInfo, SAP Data Intelligence

## Pitch

**The Question Every Buyer Asks:** *"Does my CRM fill in the blanks on missing data, or do I need to manually research every lead?"*

In a perfect world, every lead entry has the person's name, title, company, phone, email, industry, company size, and website. In reality, you get "john from acme corp" with a Gmail address. Intelligence and enrichment answer the question: **"How fast can my CRM turn bad data into good data?"** This component covers automated lead enrichment from external data sources, lead mining/discovery, and company intelligence.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM |
|---------|----------|----------|------------|------------------------|---------|---------|----------|
| Email-based enrichment | Planned | ✅ (IAP Enrich) | ✅ (Clearbit) | ✅ (LinkedIn) | ✅ | ✅ (Clearbit) | ✅ (Lead Synthesis) |
| Website-based enrichment | Planned | ✅ (iap_reveal) | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ |
| Company intelligence | Planned | ✅ (IAP Mine) | ✅ (Clearbit) | ✅ (LinkedIn) | ✅ | ✅ (Company Data) | ✅ (Verto) |
| Lead discovery/mining | Planned | ✅ (IAP Mine) | ✅ (Lead Finder) | ✅ (LinkedIn Sales) | ✅ | ✅ | ✅ |
| Role detection | Planned | ✅ (crm_iap_lead.role) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Industry detection | Planned | ✅ (crm_iap_lead.industry) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Seniority detection | Planned | ✅ (crm_iap_lead.seniority) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Auto-fill from partial data | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Data verification/bounce check | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| LinkedIn integration | Planned | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ✅ (via add-on) |
| Person-to-company match | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Enrichment history/audit | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Custom enrichment sources | Planned | ❌ | ❌ | ✅ (API) | ✅ | ❌ | ✅ (Zia) | ❌ |
| Real-time enrichment | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Enrichment batch processing | Planned | ❌ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ |

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted enrichment** — No per-enrichment fees. Enrich 100 or 100,000 leads at the cost of your infrastructure.
- **API-defined enrichment models** — Enrichment result schemas (role, industry, seniority) are OpenAPI-defined and auto-generated for all clients.
- **Rust performance** — Batch enrichment of 100,000 records via parallel HTTP calls in Rust is fast. Python-based enrichment (Odoo) is synchronous and slow.

### Where RERP Lags
- **No enrichment integrations** — No Clearbit, ZoomInfo, or Hunter API integration. The enrichment endpoints exist conceptually but no data source is connected.
- **No auto-fill from partial data** — HubSpot and Salesforce fill in missing fields automatically. RERP requires manual data entry.
- **No LinkedIn integration** — Microsoft Dynamics 365's LinkedIn integration is unmatched for B2B lead discovery.

---

## Competitive Intelligence Deep Dive

### Salesforce + Clearbit (Enterprise Enrichment — $100–$330/user/month add-on)
**Clearbit** (acquired by Salesforce) is deeply integrated into Sales Cloud. **Reveal** shows real-time company and contact data as you type — no manual lookup needed. **Enrichment** fills missing fields on every record automatically (industry, size, tech stack, social profiles). **Company Signals** show funding events, tech stack changes, employee growth, and news. **Person Signals** show social profiles, email patterns, role changes, and seniority. **Lead Finder** discovers new leads matching ICP criteria from 100M+ companies. **Email verification** validates email addresses before sending. **Data Cloud** provides real-time identity resolution across all customer touchpoints. Best for B2B enterprises with complex ICPs.

### HubSpot Lead Intelligence (SMB Enrichment — included in Professional/Enterprise tiers)
**Lead Intelligence** uses Clearbit data to auto-enrich contacts and companies on creation. **Identify** turns anonymous website visitors into identified contacts using IP matching and behavioral signals. **Lead Scoring** combines enrichment data with engagement data (page views, email opens, form fills) for a comprehensive lead quality metric. **Automatic field population** works on form submissions, contact creation, and email sends. **Contact enrichment** updates 50+ fields automatically from Clearbit's database. **Company intelligence** includes funding stage, employee count, industry, technology used, and social profiles. The simplicity is the selling point: install the CRM, and it starts filling itself in.

### Microsoft Dynamics + LinkedIn (B2B Intelligence — $30/user/month for Sales Navigator)
**LinkedIn Sales Navigator** integration provides company profiles, person profiles, org charts, and real-time job changes. **Company Insights** shows funding, headcount changes, news, and technology adoption. **Contact Intelligence** provides email patterns, phone numbers, and professional signals. **Lead Recommendations** suggests prospects based on your ICP and existing customer profiles. **Alerts** notify you when target accounts undergo changes (hiring, funding, leadership changes). **Save Searches** track prospect lists with real-time updates. **InMail** lets you message prospects directly from CRM. The LinkedIn advantage for B2B is unmatched: you get real-time org changes and job movements that signal buying intent.

### Zoho Verto + Lead Synthesis (Value Intelligence — $5–$52/user/month)
**Zoho Verto** is a built-in intelligence platform (no add-on cost). **Lead Synthesis** enriches leads with contact details (email, phone, title, social profiles) from Zoho's own database of 300M+ professionals. **Company intelligence** provides size, industry, location, revenue, and technology stack. **Lead Scoring** uses intelligence data for automated scoring alongside engagement data. **Business Card Scanner** captures contact data from physical business cards via OCR. **Email finder** discovers professional email addresses from names and domains. **Zia AI** predicts lead quality based on enrichment signals. **Lead Portals** allow prospects to self-update their information, keeping data fresh. Best value intelligence platform with no per-enrichment fees.

### Pipedrive + Clearbit (Simple Enrichment — $15–$99/user/month + Clearbit add-on)
Pipedrive integrates with **Clearbit** for basic company enrichment on contact creation. Enrichment data includes industry, company size, website, and logo. **No role/industry detection** — only basic company fields. **No LinkedIn integration**. **No lead discovery** — Pipedrive has no mining capabilities. **Data verification** via email validation. Simple enrichment for teams that don't need deep intelligence. The philosophy: "get the basics, use your own research for the rest."

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 3-4 weeks)
1. Define `EnrichmentRequest` entity: id, email, website, company_name, status, created_at
2. Define `EnrichmentResult` entity: id, request_id, industry, size, website, location, tech_stack
3. Define `LeadMiningRequest` entity: id, criteria, status, results_count, created_at
4. Create enrichment endpoint (POST /leads/{id}/enrich)
5. Implement enrichment batch endpoint (POST /enrichment/batch)

### Phase 2 (4-8 weeks)
1. Integrate Clearbit API (or alternative: Hunter, Apollo)
2. Integrate ZoomInfo API (for enterprise data)
3. Auto-fill endpoint (POST /leads/auto-fill?email=john@acme.com)
4. Data verification endpoint (POST /verify/email)
5. Enrichment history audit endpoint

### Phase 3 (8-12 weeks)
1. LinkedIn API integration
2. Real-time enrichment (as-you-type auto-fill)
3. Company-to-person matching
4. Custom enrichment source plugin system
5. Enrichment analytics (data quality score)

---

## Key Takeaway for Buyers

Enrichment is the difference between a CRM that requires manual data entry and one that fills itself in. A buyer with a lead list of 50,000 contacts with missing fields needs enrichment to be viable. RERP's self-hosted approach means enrichment costs scale with infrastructure, not with the number of enrichments. The critical gap: no data source integration yet. Once Clearbit or similar is connected, RERP becomes competitive on enrichment for cost-sensitive organizations.
