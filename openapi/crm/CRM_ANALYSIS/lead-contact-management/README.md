# Lead & Contact Management

> **Component:** Core CRM entity lifecycle — capture, unify, enrich, govern
> **Priority:** P0 — Foundation layer; everything else depends on this
> **Odoo Reference:** crm.lead (2,877 lines), res.partner (10,000+ lines), crm_merge_wizard

---

## The Pitch

**Buyer Question:** *Can I capture every person and organization that touches my business, unify them into a single source of truth, and govern their data from first touch to forever?*

If the answer is no, you don't have a CRM — you have a collection of spreadsheets. Lead and contact management is the foundation. Without a complete, unified data model, nothing else works: no pipeline, no revenue, no reporting, no automation. This component defines how prospects become contacts, how contacts link to accounts, how records are deduplicated and enriched, and how compliance is enforced.

---

## What This Component Does

Lead & Contact Management is the system of record for every human and organization in your business. It handles the full lifecycle:

1. **Capture** — Leads enter from web forms, email, imports, APIs, livechat, events, social media, or manual entry
2. **Unify** — Multiple records for the same person/company are merged automatically (deduplication)
3. **Enrich** — Missing fields are filled from external data sources (Clearbit, ZoomInfo, Hunter)
4. **Convert** — A lead becomes a contact (person) and/or an account (company)
5. **Govern** — GDPR blacklisting, consent management, right-to-be-forgotten, data retention
6. **Link** — Contacts link to accounts, accounts link to opportunities, opportunities link to deals
7. **Search** — Fast, trigram-indexed search across all records

---

## Entity Model: The Unified Lead/Opportunity Record

Odoo's approach is the gold standard for simplicity: **one model, two modes** via a `type` field. RERP's current design (three separate entities with no relationships) is fundamentally flawed. Here's what the unified model needs:

### Core Lead/Opportunity Entity

| Field | Type | Required | Tracked | Purpose |
|-------|------|----------|---------|---------|
| `id` | UUID | Yes | No | Primary key |
| `name` | String (255) | Yes | No | Lead/opportunity title; trigram-indexed |
| `type` | Enum: [LEAD, OPPORTUNITY] | Yes | No | Lead (qualification mode) vs Opportunity (pipeline mode) |
| `email_from` | String (255) | No | Yes | Primary email; normalized form for dedup |
| `email_normalized` | String (255) | Computed | No | Lowercase, stripped — for dedup comparison |
| `phone` | String (64) | No | Yes | Primary phone; sanitized form for dedup |
| `phone_sanitized` | String (64) | Computed | No | Digits only; trigram-indexed |
| `mobile` | String (64) | No | Yes | Mobile number |
| `title` | String (64) | No | No | Salutation: Mr/Mrs/Mlle (Many2one to res.partner.title) |
| `function` | String (128) | No | No | Job title/position |
| `contact_name` | String (255) | No | No | Contact person name (for B2B leads) |
| `company_name` | String (255) | No | No | Company name on lead (pre-creation) |
| `website` | String (255) | No | No | Company website |
| `description` | Text | No | Yes | Internal notes; HTML-formatted |
| `referred_by` | String (255) | No | No | Referred by whom |
| `priority` | Enum: [LOW, NORMAL, HIGH, URGENT] | No | No | Urgency: 0, 1, 2, 3 |
| `active` | Boolean | No | No | Soft delete (default: true) |
| `is_blacklisted` | Boolean | No | Yes | GDPR opt-out |
| `color` | Integer | No | No | Kanban color (1-16) |

### Pipeline Entity Fields

| Field | Type | Purpose |
|-------|------|---------|
| `stage_id` | Foreign Key: Stage | Current stage in pipeline |
| `tag_ids` | Many2Many: Tag | Classification tags (Sales, Enterprise, Channel) |
| `probability` | Float (0-100) | Manual win probability |
| `automated_probability` | Float (0-100) | Bayesian AI-computed probability |
| `is_automated_probability` | Boolean | Override flag — use AI score or manual? |
| `date_deadline` | Date | Expected close date |
| `date_open` | DateTime | When lead was assigned/opened |
| `date_closed` | DateTime | When lead was won or lost |
| `date_last_stage_update` | DateTime | Last time stage changed |
| `day_open` | Float | Days from create to assign |
| `day_close` | Float | Days from open to close |

### Revenue Entity Fields

| Field | Type | Purpose |
|-------|------|---------|
| `expected_revenue` | Decimal (15,2) | Expected deal value |
| `prorated_revenue` | Decimal (15,2) | Revenue × probability (computed) |
| `recurring_revenue` | Decimal (15,2) | Monthly recurring revenue |
| `recurring_plan_id` | Foreign Key: RecurringPlan | Plan type (monthly, quarterly, annual) |
| `recurring_revenue_monthly` | Decimal (15,2) | MRR (computed) |
| `recurring_revenue_monthly_prorated` | Decimal (15,2) | Prorated MRR (computed) |
| `recurring_revenue_prorated` | Decimal (15,2) | Total prorated revenue (computed) |
| `company_currency_id` | Foreign Key: Currency | Base currency |

### Partner/Contact Linkage

| Field | Type | Purpose |
|-------|------|---------|
| `partner_id` | Foreign Key: res.partner | Linked contact record |
| `partner_is_blacklisted` | Boolean | GDPR opt-out inherited from partner |
| `commercial_partner_id` | Foreign Key: res.partner | Parent company partner |
| `company_id` | Foreign Key: res.company | Parent company (computed) |
| `user_id` | Foreign Key: res.users | Assigned salesperson (tracked) |
| `team_id` | Foreign Key: crm.team | Sales team (tracked) |
| `user_company_ids` | Many2Many: res.company | Multi-company visibility |

### UTM Campaign Tracking

| Field | Type | Purpose |
|-------|------|---------|
| `campaign_id` | Foreign Key: utm.campaign | Campaign source |
| `medium_id` | Foreign Key: utm.medium | Medium (email, cpc, social, organic) |
| `source_id` | Foreign Key: utm.source | Source (google, linkedin, referral) |

### Date Tracking

| Field | Type | Purpose |
|-------|------|---------|
| `create_date` | DateTime | Auto-set on creation |
| `create_uid` | Foreign Key: res.users | User who created |
| `write_date` | DateTime | Last modified |
| `write_uid` | Foreign Key: res.users | User who last modified |
| `date_conversion` | DateTime | When lead converted to opportunity |
| `date_automation_last` | DateTime | Last automated action |

### Deduplication

| Field | Type | Purpose |
|-------|------|---------|
| `duplicate_lead_ids` | Many2Many: crm.lead | Linked duplicate records |
| `duplicate_lead_count` | Integer | Count of duplicates |

**Total fields: ~70.** This is the entity that everything else depends on. Every other entity in RERP CRM either links to this or derives from it.

---

## Account & Contact Entity

Lead management is personal; Account management is organizational. The Account (Company) model:

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `name` | String (255) | Company name; trigram-indexed |
| `email` | String (255) | Company email |
| `phone` | String (64) | Company phone |
| `website` | String (255) | Company URL |
| `industry_id` | Foreign Key: industry | Industry classification |
| `company_size` | Enum: [SMB, Mid-Market, Enterprise] | Company size |
| `employees` | Integer | Employee count |
| `annual_revenue` | Decimal (15,2) | Company revenue |
| `description` | Text | Company description |
| `street`, `street2`, `city`, `state_id`, `zip`, `country_id` | Various | Full address |
| `parent_id` | Foreign Key: Account | Parent company |
| `child_ids` | One2Many: Account | Sub-companies |
| `partner_id` | Foreign Key: res.partner | Linked partner record |
| `active` | Boolean | Soft delete |
| `type` | Enum: [CONTACT, INSTALLATION, ADDRESS] | Partner role |

Contact entity (res.partner) links people to accounts:

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `name` | String (255) | Full name |
| `email` | String (255) | Email |
| `phone`, `mobile` | String | Phone numbers |
| `title` | Foreign Key: res.partner.title | Mr/Mrs/Mlle |
| `function` | String | Job title |
| `department` | String | Department |
| `company_id` | Foreign Key: Account | Parent company |
| `street`...`country_id` | Various | Personal address |
| `user_ids` | Many2Many: res.users | Users with access |
| `is_company` | Boolean | Person vs company distinction |
| `parent_id` | Foreign Key: Contact | Parent contact (for org structure) |

---

## Entity Relationships

```
crm.lead (unified lead/opportunity)
  ├── res.partner (contact)          ← via partner_id
  ├── crm.account (account)          ← via company_id
  ├── crm.stage (pipeline)           ← via stage_id
  ├── crm.team (team)                ← via team_id
  ├── crm.recurring_plan (revenue)   ← via recurring_plan_id
  ├── utm.campaign (marketing)       ← via campaign_id
  ├── crm.tag (classification)       ← via tag_ids
  └── crm.lead (self)                ← via duplicate_lead_ids

crm.tag (classification)
  └── crm.lead × Many2Many (tag_ids)

utm.campaign → utm.medium → utm.source (hierarchy)
```

---

## Required API Endpoints

### Lead/Opportunity CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leads` | List leads with filters, pagination, search |
| `GET` | `/leads/{id}` | Get lead detail with all related entities |
| `POST` | `/leads` | Create a new lead or opportunity |
| `PATCH` | `/leads/{id}` | Update lead fields |
| `DELETE` | `/leads/{id}` | Soft-delete (set active=false) |
| `GET` | `/leads/search/{query}` | Trigram full-text search |

### Lead Conversion

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/leads/{id}/convert` | Convert lead to contact + account |
| `POST` | `/leads/convert-batch` | Convert multiple leads at once |

### Lead Merge

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/leads/{id}/merge` | Merge two lead records |
| `POST` | `/leads/detect-duplicates` | Detect duplicates for a lead |
| `GET` | `/leads/{id}/duplicates` | List duplicate records |

### Contact/Account CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/contacts` | List contacts |
| `POST` | `/contacts` | Create contact |
| `GET` | `/accounts` | List accounts |
| `POST` | `/accounts` | Create account |

### Search & Discovery

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leads/search` | Full-text search with filters |
| `POST` | `/leads/import` | Bulk import from CSV/JSON |
| `GET` | `/leads/export` | Bulk export to CSV/JSON |
| `GET` | `/contacts/search` | Contact search with filters |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Unified Model with Type Field
Odoo's `crm.lead` handles both leads and opportunities via a single model. The `type` field controls behavior — lead mode shows qualification fields, opportunity mode shows pipeline fields. **Recommendation: RERP should use a single Lead entity with a `type` field, NOT separate Lead and Opportunity entities.** This eliminates conversion copying and simplifies reporting.

### Pattern 2: Mixin Inheritance for Cross-Cutting Concerns
Odoo uses mixin models to add common features:
- `mail.thread.cc` — CC on emails
- `mail.thread.blacklist` — GDPR opt-out
- `mail.thread.phone` — Phone validation
- `mail.activity.mixin` — Follow-up activities
- `utm.mixin` — Campaign tracking
- `format.address.mixin` — Address formatting
- `mail.tracking.duration.mixin` — Stage transition timing

**Recommendation: RERP should define reusable field groups in OpenAPI that get composed into the Lead schema.** Instead of duplicating `campaign_id`, `medium_id`, `source_id` across entities, define them once in a UTM section.

### Pattern 3: Trigram Indexing for Search
Odoo uses `GIN` indexes with `trigram` extension on `name`, `email_from`, and `phone_sanitized`. This enables fuzzy search: "john acme corp" finds "John Smith @ Acme Corporation" even with typos. **Recommendation: Define trigram indexes in the migration layer for name, email_normalized, and phone_sanitized.**

### Pattern 4: Normalized Email for Dedup
Odoo maintains `email_normalized` (lowercase, stripped) alongside the raw email. Deduplication compares normalized emails. **Recommendation: Always store both raw and normalized forms. Index normalized form.**

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-first, machine-readable data model** — Every entity, field, and relationship is defined in OpenAPI specs, enabling automatic SDK generation, API contracts, and tooling. No other CRM exposes its data model this cleanly.
- **Rust-based search performance** — Trigram search across 1 million records in Rust is instantaneous. Python-based dedup (Odoo) can be slow at scale.
- **Self-hosted, no vendor lock-in** — No per-seat pricing, no rate limits, no data egress fees.

### Where RERP Lags
- **Schema definitions are empty** — Zero fields defined across all entities. This is the critical first deliverable.
- **Three separate entities with no relationships** — Flawed design. Needs unified model.
- **No deduplication algorithm** — No email/phone normalization, no fuzzy matching.
- **No conversion flow** — No way to convert leads to contacts/accounts.
- **No activity timeline** — No chronological view of all interactions per record.

---

## Competitive Intelligence Deep Dive

### Salesforce: Account-Contact-Opportunity Trinity
The holy trinity: Account (company) → Contacts (people) → Opportunities (deals). Contact Role on Opportunity allows multiple stakeholders per deal. **Duplicate Rules** enforce prevention before creation. **Einstein Data Cloud** provides real-time identity resolution. **Lead Score** (1-100) weighs 200+ signals. Custom objects extend to 10,000+ fields.

### HubSpot: Frictionless Entry
2,000+ custom properties per contact. Clearbit auto-enrichment on creation. Identify turns anonymous visitors into contacts via IP matching. Contact merging detects duplicates automatically. The free tier is a trap: once data is in, migration is impossible.

### Microsoft: LinkedIn Integration
Dynamics 365 + LinkedIn Sales Navigator = unmatched B2B intelligence. Real-time org changes (hiring, funding, leadership) signal buying intent. Email routing via Outlook is invisible — no manual logging.

### Zoho: Best Value
Lead Synthesis (300M+ professional database), business card OCR, lead portals for self-update. Zia AI scoring at $5/user/month. Canvas lets users redesign layouts without code.

### Pipedrive: Minimalist
Contacts and companies only — no accounts, no opportunities. The pipeline IS the CRM. Good for simple sales teams, insufficient for complex B2B.

---

## Implementation Roadmap

### Phase 1: Core Schema (2-3 weeks) — P0
1. Define unified `Lead` entity with all ~70 fields from Odoo model
2. Define `Contact` and `Account` entities with partner linkage
3. Add `email_normalized`, `phone_sanitized` for dedup
4. Establish relationships: Lead → Contact → Account via foreign keys
5. Implement basic email-based deduplication endpoint
6. Define `Stage`, `Tag`, `Campaign`, `Medium`, `Source` supporting entities

### Phase 2: Conversion & Merge (2-4 weeks) — P0
1. Lead-to-Contact conversion endpoint (update `type`, optionally create partner)
2. Lead merge endpoint with field-level conflict resolution (winner selection)
3. Duplicate detection endpoint (fuzzy email + phone matching)
4. Bulk import from CSV/JSON endpoint
5. Bulk export to CSV/JSON endpoint

### Phase 3: Enrichment & Governance (2-4 weeks) — P1
1. GDPR blacklisting endpoint and consent fields
2. Contact enrichment via external API (Clearbit/Hunter)
3. Activity timeline endpoint (aggregate all interactions per lead)
4. Custom fields schema (key-value store per entity type)
5. File attachment management endpoint

### Phase 4: Search & Discovery (2-3 weeks) — P1
1. Trigram full-text search endpoint
2. Fuzzy match on name, email, phone
3. Smart suggestions (autocomplete for company, industry)
4. Advanced filters (multi-select, date ranges, field-based)

---

## Key Takeaway for Buyers

RERP CRM's pitch is **open, fast, and self-hosted** with an **OpenAPI-first data model** that no other CRM can match. If your organization values data sovereignty, API contracts, and Rust-native performance over out-of-the-box drag-and-drop, RERP is the compelling choice.

The gap with Salesforce/HubSpot is the entire data model — 70+ fields, relationships, and business logic that Odoo ships with out of the box. But RERP's advantage is that once defined, every client gets the complete model automatically via code generation. No vendor release cycle needed.

**The immediate priority: define the unified Lead entity with all fields, then build the conversion and merge flows. Everything else depends on this foundation.**
