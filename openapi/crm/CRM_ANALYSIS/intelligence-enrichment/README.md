# Intelligence & Enrichment

> **Component:** Automated data enrichment, lead discovery, and company intelligence
> **Priority:** P4 — Nice-to-have; can integrate later
> **Odoo Reference:** crm_iap_lead (IAP enrichment), crm_iap_lead_merge (merge), crm_iap_lead_personal_info_access (personal info)

---

## The Pitch

**Buyer Question:** *Does my CRM fill in the blanks on missing data, or do I need to manually research every lead?*

In a perfect world, every lead entry has the person's name, title, company, phone, email, industry, company size, and website. In reality, you get "john from acme corp" with a Gmail address. Intelligence and enrichment answer the question: **how fast can my CRM turn bad data into good data?** This component covers automated lead enrichment from external data sources, lead mining/discovery, and company intelligence.

---

## What This Component Does

1. **Email-Based Enrichment** — Submit an email address → get back company, title, phone, social profiles
2. **Website-Based Enrichment** — Submit a company URL → get back industry, size, tech stack, revenue
3. **Company Intelligence** — Deep company data: funding rounds, tech stack, employee growth, news
4. **Lead Discovery** — Find new leads matching ICP criteria from large databases
5. **Role/Seniority Detection** — Automatically detect job level and role from name + company
6. **Auto-Fill from Partial Data** — Type a partial email or name → CRM fills in the rest
7. **Data Verification** — Validate email addresses, phone numbers before sending
8. **LinkedIn Integration** — Pull company and person profiles from LinkedIn
9. **Enrichment History** — Track what data was enriched, when, and from which source
10. **Batch Enrichment** — Enrich 10,000 records in one API call
11. **Real-Time Enrichment** — As-you-type auto-fill in the UI
12. **Person-to-Company Matching** — Automatically link contacts to correct accounts

---

## Entity Model

### Enrichment Request Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `lead_id` | Foreign Key: Lead | No | Lead being enriched (if any) |
| `contact_id` | Foreign Key: Contact | No | Contact being enriched |
| `email` | String (255) | No | Email to enrich |
| `company_name` | String (255) | No | Company name to enrich |
| `website` | String (255) | No | Company website to enrich |
| `status` | Enum: [PENDING, SUCCESS, FAILED] | Yes | Processing status |
| `source` | Enum: [CLEARBIT, HUNTER, ZOOMINFO, APOLLO, LINKEDIN] | Yes | Enrichment data source |
| `request_data` | JSON | Yes | Original data submitted for enrichment |
| `created_at` | DateTime | Yes | When request was made |
| `completed_at` | DateTime | No | When enrichment completed |

### Enrichment Result Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `enrichment_request_id` | Foreign Key: EnrichmentRequest | Yes | Associated request |
| `industry` | String (128) | No | Company industry |
| `size` | Enum: [1-10, 11-50, 51-200, 201-500, 501-1000, 1001-5000, 5001-10000, 10000+] | No | Employee count range |
| `website` | String (255) | No | Company website |
| `location` | String (255) | No | HQ location |
| `country` | String (64) | No | Country code |
| `tech_stack` | JSON | No | Technologies used (array of strings) |
| `funding_total` | Decimal(15,2) | No | Total funding raised |
| `funding_round` | String (64) | No | Last funding round (Series A, B, C, IPO) |
| `employee_count` | Integer | No | Exact employee count |
| `annual_revenue` | Decimal(15,2) | No | Company revenue |
| `linkedin_url` | String (512) | No | Company LinkedIn URL |
| `twitter_url` | String (512) | No | Company Twitter URL |
| `facebook_url` | String (512) | No | Company Facebook URL |
| `logo_url` | String (512) | No | Company logo URL |
| `person_name` | String (255) | No | Person's full name |
| `person_title` | String (255) | No | Job title |
| `person_role` | Enum: [EXECUTIVE, MANAGER, INDIVIDUAL_CONTRIBUTOR, INTERN, OTHER] | No | Role level |
| `person_seniority` | Enum: [C_LEVEL, VP, DIRECTOR, MANAGER, SENIOR, MID, JUNIOR] | No | Seniority level |
| `person_email` | String (255) | No | Validated email |
| `person_phone` | String (64) | No | Work phone |
| `personlinkedin_url` | String (512) | No | Person LinkedIn URL |
| `data_freshness` | Enum: [REALTIME, RECENT, OLD, UNKNOWN] | No | How fresh the data is |

### Lead Mining Request Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `criteria` | JSON | Yes | Search criteria (industry, size, location, title, etc.) |
| `status` | Enum: [PENDING, SUCCESS, FAILED] | Yes | Processing status |
| `results_count` | Integer | No | Number of leads found |
| `source` | Enum: [CLEARBIT, ZOOMINFO, APOLLO, LINKEDIN] | Yes | Data source |
| `created_at` | DateTime | Yes | When request was made |
| `completed_at` | DateTime | No | When mining completed |

### Data Verification Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `email` | String (255) | Yes | Email to verify |
| `phone` | String (64) | No | Phone to verify |
| `email_valid` | Boolean | No | Is email valid? |
| `email_disposable` | Boolean | No | Is email a disposable address? |
| `email_role_based` | Boolean | No | Is email role-based (info@, admin@)? |
| `phone_valid` | Boolean | No | Is phone valid? |
| `phone_carrier` | String (128) | No | Phone carrier |
| `phone_type` | Enum: [MOBILE, LANDLINE, VOIP] | No | Phone type |
| `verified_at` | DateTime | No | When verification completed |

---

## Enrichment API Patterns

### Pattern 1: Email-Based Enrichment
```
POST /enrichment/lookup?email=john@acme.com

Returns:
{
  "company": {
    "name": "Acme Corporation",
    "industry": "Technology",
    "size": "501-1000",
    "website": "https://acme.com",
    "location": "San Francisco, CA",
    "tech_stack": ["salesforce", "aws", "react"],
    "funding_total": 50000000,
    "funding_round": "Series B",
    "logo_url": "https://logo.clearbit.com/acme.com"
  },
  "person": {
    "name": "John Smith",
    "title": "VP of Engineering",
    "role": "EXECUTIVE",
    "seniority": "VP",
    "email": "john.smith@acme.com",
    "email_valid": true,
    "phone": "+1-555-123-4567",
    "linkedin_url": "https://linkedin.com/in/johnsmith"
  }
}
```

### Pattern 2: Auto-Fill from Partial Data
```
POST /enrichment/auto-fill?email=john@acme.com

Same as lookup, but:
1. Checks local database first (no API call needed)
2. Only calls external API if local data is stale or incomplete
3. Updates local record with new data
4. Returns diff of what was changed
```

### Pattern 3: Batch Enrichment
```
POST /enrichment/batch

Input: List of emails [{email: "..."}, {email: "..."}, ...]
Returns: EnrichmentResult[] for each email

Process:
1. Queue each enrichment request
2. Process in parallel (async Rust HTTP calls)
3. Return results as they complete
4. Store results in EnrichmentResult table
5. Update source records with new data
```

---

## Required API Endpoints

### Enrichment

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/enrichment/lookup` | Enrich single lead by email |
| `POST` | `/enrichment/website` | Enrich by company website |
| `POST` | `/enrichment/batch` | Batch enrich N records |
| `POST` | `/enrichment/auto-fill` | Auto-fill from partial data |
| `GET` | `/enrichment/{id}/result` | Get enrichment result |
| `GET` | `/enrichment/history` | Enrichment history for a lead |

### Verification

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/verify/email` | Verify single email |
| `POST` | `/verify/batch` | Verify list of emails |
| `POST` | `/verify/phone` | Verify phone number |

### Lead Mining

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/mining/search` | Search for leads matching criteria |
| `POST` | `/mining/icp-match` | Find leads matching Ideal Customer Profile |
| `GET` | `/mining/{id}/results` | Get mining results |

### Data Quality

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/data-quality/score` | Overall data quality score |
| `GET` | `/data-quality/missing` | Records with missing key fields |
| `GET` | `/data-quality/duplicates` | Duplicate detection report |

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted enrichment** — No per-enrichment fees. Enrich 100 or 100,000 leads at the cost of infrastructure.
- **API-defined enrichment models** — Enrichment result schemas are OpenAPI-defined and auto-generated for all clients.
- **Rust performance** — Batch enrichment of 100,000 records via parallel HTTP calls in Rust is fast. Python (Odoo) is synchronous and slow.

### Where RERP Lags
- **No enrichment integrations** — No Clearbit, ZoomInfo, or Hunter API integration.
- **No auto-fill from partial data** — Manual data entry required today.
- **No LinkedIn integration** — Microsoft Dynamics' LinkedIn integration is unmatched for B2B.

---

## Competitive Intelligence Deep Dive

### Salesforce + Clearbit (Enterprise — $100–$330/user/month add-on)
**Clearbit Reveal** shows real-time company data as you type. **Enrichment** fills missing fields automatically. **Company Signals** show funding events, tech stack changes. **Lead Finder** discovers new leads from 100M+ companies. **Email verification** validates addresses. Best for B2B enterprises with complex ICPs.

### HubSpot Lead Intelligence (included in Pro/Ent)
Clearbit auto-enrichment on creation. **Identify** turns anonymous visitors into contacts. **Lead Scoring** combines enrichment with engagement data. **Automatic field population** works on form submissions. Install CRM, and it starts filling itself in.

### Microsoft + LinkedIn (B2B — $30/user/month)
**LinkedIn Sales Navigator** provides company profiles, org charts, real-time job changes. **Company Insights** shows funding, headcount changes, news. **Lead Recommendations** suggests prospects based on ICP. **Alerts** notify on target account changes. Unmatched for B2B buying intent signals.

### Zoho Verto (Value — included with CRM)
Built-in intelligence platform (no add-on cost). **Lead Synthesis** enriches from 300M+ professional database. **Business Card Scanner** via OCR. **Email finder** discovers addresses from names and domains. No per-enrichment fees.

---

## Implementation Roadmap

### Phase 1: Enrichment Foundation (3-4 weeks)
1. Define `EnrichmentRequest` and `EnrichmentResult` entities
2. Define `LeadMiningRequest` entity
3. Create enrichment endpoint (POST /enrichment/lookup)
4. Implement enrichment batch endpoint
5. Add enrichment fields to Contact/Account entities

### Phase 2: Integrations (4-6 weeks)
1. Integrate Clearbit API (primary enrichment source)
2. Integrate Hunter API (email discovery)
3. Auto-fill endpoint (POST /enrichment/auto-fill)
4. Data verification endpoint (POST /verify/email)
5. Enrichment history audit endpoint

### Phase 3: Advanced (4-6 weeks)
1. LinkedIn API integration
2. Real-time enrichment (as-you-type auto-fill)
3. Company-to-person matching
4. Custom enrichment source plugin system
5. Enrichment analytics (data quality score)

---

## Key Takeaway for Buyers

Enrichment is the difference between a CRM that requires manual data entry and one that fills itself in. RERP's self-hosted approach means enrichment costs scale with infrastructure, not with the number of enrichments. The critical gap: no data source integration yet. Once Clearbit or similar is connected, RERP becomes competitive on enrichment for cost-sensitive organizations.
