# RERP CRM vs Odoo CRM: Deep-Dive Field-by-Field Comparison

> **Date:** 2026-05-10  
> **Sources:** RERP OpenAPI specs, Odoo Community v19.0 CRM (2,877 lines crm_lead.py + 6 additional model files)  
> **Scope:** Field-by-field, model-by-model, business logic analysis

---

## 1. Architecture Comparison: The Fundamental Difference

### RERP: API-First, Empty Shells

```
openapi.yaml (gateway) ───┐
  ├── core/openapi.yaml ───┼── 24 endpoint definitions
  ├── automation/ ────────┐│   0 schema definitions
  └── livechat/ ──────────┘│
```

RERP has the **right structure** but zero content. Every `schemas: {}` is empty. The endpoint paths exist but without any request/response bodies, no field types, no validation, no relationships.

### Odoo: Model-First, Complete System

```
crm_lead.py          ─── 2,877 lines, ~120 field definitions
crm_team.py          ───   759 lines,  assignment logic
crm_stage.py         ───   188 lines,  pipeline stages
crm_team_member.py   ───    99 lines,  load-based routing
crm_recurring_plan.py──    20 lines,   revenue plans
crm_lost_reason.py   ───    33 lines,  loss tracking
crm_scoring_freq.py  ───    30 lines,  Bayesian scoring
+ 7 data files ──── seed stages, scoring fields, cron jobs
+ 6 wizard files ── lead conversion, merge, scoring, lost reasons
+ 2 XML security files ── 40+ permission records
```

Odoo ships with **complete business logic**: assignment algorithms, probability computation, partner sync, phone/email validation, scoring frequency tables, activity planning — all implemented.

---

## 2. Entity Model: RERP vs Odoo

### 2.1 The Lead/Opportunity Model

**RERP defines three separate entities with no relationships:**

| Entity | Fields | Schema Status |
|--------|--------|---------------|
| `Lead` | None (empty) | `schemas: {}` |
| `Contact` | None (empty) | `schemas: {}` |
| `Opportunity` | None (empty) | `schemas: {}` |

Three independent resources. No way to convert a Lead to an Opportunity. No way to link an Opportunity to a Contact or Account. No shared fields.

**Odoo uses ONE unified model with a type field:**

```python
class CrmLead(models.Model):
    _name = 'crm.lead'  # Handles BOTH leads and opportunities
    _inherit = [
        'mail.thread.cc',        # CC on emails
        'mail.thread.blacklist', # GDPR opt-out
        'mail.thread.phone',     # Phone validation
        'mail.activity.mixin',   # Follow-up activities
        'utm.mixin',             # Campaign/medium/source tracking
        'format.address.mixin',  # Address formatting
        'mail.tracking.duration.mixin',  # Stage transition timing
    ]
```

**Type differentiation:** `type = Selection([('lead','Lead'), ('opportunity','Opportunity')])` — one model, two modes.

**Why this matters:** In RERP, converting a lead to an opportunity requires copying data between three separate tables. In Odoo, it's a wizard that updates the `type` field and optionally creates a partner record. The unified model means every lead automatically inherits communication tracking, UTM tracking, activity planning, and partner sync.

### 2.2 Complete Field Comparison: Lead/Opportunity

| Category | Field | RERP | Odoo | Purpose |
|----------|-------|------|------|---------|
| **Identity** | `name` | Missing | Char, trigram-indexed, required | Opportunity/lead title |
| **Identity** | `email_from` | Missing | Char, trigram-indexed, normalized | Primary email |
| **Identity** | `phone` | Missing | Char, sanitized, validated | Primary phone |
| **Identity** | `mobile` | Missing | Char | Mobile number |
| **Identity** | `title` | Missing | Char (Many2one to res.partner.title) | Salutation (Mr/Mrs/Mlle) |
| **Identity** | `function` | Missing | Char | Job title/position |
| **Identity** | `contact_name` | Missing | Char | Contact person name |
| **Identity** | `partner_name` | Missing | Char | Company name (on lead) |
| **Identity** | `website` | Missing | Char | Company website |
| **Identity** | `lang_id` | Missing | Many2one to res.lang | Language preference |
| **Salesperson** | `user_id` | Missing | Many2one res.users, tracked | Assigned rep |
| **Team** | `team_id` | Missing | Many2one crm.team, tracked | Sales team |
| **Team** | `user_company_ids` | Missing | Many2many res.company | Company visibility |
| **Company** | `company_id` | Missing | Many2one res.company, computed | Parent company |
| **Partner** | `partner_id` | Missing | Many2one res.partner, tracked | Linked contact |
| **Partner** | `commercial_partner_id` | Missing | Many2one res.partner | Parent company |
| **Partner** | `partner_is_blacklisted` | Missing | Boolean | GDPR opt-out |
| **Type** | `type` | N/A | Selection (lead/opportunity) | Lead vs opportunity |
| **Type** | `active` | Missing | Boolean, default True | Soft delete |
| **Description** | `description` | Missing | Html | Internal notes |
| **Description** | `referred` | Missing | Char | Referred by |
| **Priority** | `priority` | Missing | Selection 0-3 (Low→VeryHigh) | Urgency |
| **Priority** | `color` | Missing | Integer | Kanban color |
| **Pipeline** | `stage_id` | Missing | Many2one crm.stage | Current stage |
| **Pipeline** | `stage_id_color` | Missing | Integer (related) | Stage color |
| **Pipeline** | `tag_ids` | Missing | Many2many crm.tag | Classification tags |
| **Revenue** | `expected_revenue` | Missing | Monetary | Deal value |
| **Revenue** | `prorated_revenue` | Missing | Monetary, computed | Revenue × probability |
| **Revenue** | `recurring_revenue` | Missing | Monetary | Monthly recurring |
| **Revenue** | `recurring_plan` | Missing | Many2one crm.recurring.plan | Plan type |
| **Revenue** | `recurring_revenue_monthly` | Missing | Monetary, computed | MRR |
| **Revenue** | `recurring_revenue_monthly_prorated` | Missing | Monetary, computed | Prorated MRR |
| **Revenue** | `recurring_revenue_prorated` | Missing | Monetary, computed | Total prorated |
| **Currency** | `company_currency` | Missing | Many2one res.currency | Base currency |
| **Probability** | `probability` | Missing | Float 0-100, avg-aggregated | Manual win % |
| **Probability** | `automated_probability` | Missing | Float, computed | AI score |
| **Probability** | `is_automated_probability` | Missing | Boolean | Override flag |
| **Dates** | `create_date` | Missing | Datetime (auto) | Creation date |
| **Dates** | `date_open` | Missing | Datetime, computed | Assignment date |
| **Dates** | `day_open` | Missing | Float, computed | Days to assign |
| **Dates** | `date_closed` | Missing | Datetime, computed | Closed date |
| **Dates** | `date_last_stage_update` | Missing | Datetime, computed | Last stage change |
| **Dates** | `date_conversion` | Missing | Datetime | Lead→Opp conversion |
| **Dates** | `day_close` | Missing | Float, computed | Days to close |
| **Dates** | `date_deadline` | Missing | Date | Expected close |
| **Dates** | `date_automation_last` | Missing | Datetime | Last auto action |
| **Addresses** | `street` | Missing | Char | Street address |
| **Addresses** | `street2` | Missing | Char | Street line 2 |
| **Addresses** | `city` | Missing | Char | City |
| **Addresses** | `state_id` | Missing | Many2one res.country.state | State |
| **Addresses** | `zip` | Missing | Char | Postal code |
| **Addresses** | `country_id` | Missing | Many2one res.country | Country |
| **Email Quality** | `email_state` | Missing | Selection (correct/incorrect) | Email validation |
| **Email Quality** | `email_normalized` | Missing | Char (btree-indexed) | Normalized email |
| **Email Quality** | `email_domain_criterion` | Missing | Char (btree) | Email domain for dedup |
| **Phone Quality** | `phone_state` | Missing | Selection (correct/incorrect) | Phone validation |
| **Phone Quality** | `phone_sanitized` | Missing | Char (btree-indexed) | Sanitized phone |
| **Lost** | `won_status` | Missing | Selection (won/lost/pending) | Win/lost status |
| **Lost** | `lost_reason_id` | Missing | Many2one crm.lost.reason | Loss cause |
| **Duplicates** | `duplicate_lead_ids` | Missing | Many2many crm.lead | Duplicate detection |
| **Duplicates** | `duplicate_lead_count` | Missing | Integer | Duplicate count |
| **Activity** | `calendar_event_ids` | Missing | One2many calendar.event | Scheduled meetings |
| **Custom** | `lead_properties` | Missing | Properties (polymorphic) | Custom key-value fields |
| **UTM** | `campaign_id` | Missing | Many2one utm.campaign | Campaign tracking |
| **UTM** | `medium_id` | Missing | Many2one utm.medium | Medium tracking |
| **UTM** | `source_id` | Missing | Many2one utm.source | Source tracking |

**Summary:** Odoo has **~70 fields** on crm.lead. RERP has **0 fields** (schemas are empty). This is not a "gap" — it's the entire data model.

---

## 3. Entity Model: Teams

### 3.1 Sales Team

**RERP:** No Team entity exists in any spec. No team_id on Leads. No assignment logic.

**Odoo:** Full team model with 759 lines of business logic.

| Field | Purpose |
|-------|---------|
| `name` | Team name |
| `use_leads` | Enable lead qualification mode |
| `use_opportunities` | Enable pipeline mode |
| `alias_id` | Email-to-lead alias (e.g., sales@company.com) |
| `alias_name` | Email alias (e.g., "info") |
| `alias_defaults` | JSON defaults for incoming emails |
| `assignment_enabled` | Auto-assignment active |
| `assignment_auto_enabled` | Cron auto-assignment active |
| `assignment_optout` | Team skips auto-assignment |
| `assignment_max` | Monthly capacity (sum of members' max) |
| `assignment_domain` | Filter domain for lead routing |
| `lead_properties_definition` | Custom field definitions for leads |

### 3.2 Team Member

**RERP:** No TeamMember entity.

**Odoo:** 99 lines with load-based assignment.

| Field | Purpose |
|-------|---------|
| `user_id` | Assigned salesperson |
| `crm_team_id` | Parent team |
| `assignment_enabled` | Member can receive leads |
| `assignment_domain` | Domain filter for this member |
| `assignment_domain_preferred` | Preferred domain (higher priority) |
| `assignment_optout` | Pause assignments to this member |
| `assignment_max` | Average leads capacity (30 days) |
| `lead_day_count` | Leads assigned in last 24h |
| `lead_month_count` | Leads assigned in last 30 days |

### 3.3 Assignment Algorithm (Odoo)

The assignment algorithm is sophisticated:

1. **Cron runs daily** (`_cron_assign_leads`)
2. **Team allocation:** `_allocate_leads()` distributes unassigned leads to teams based on:
   - `assignment_domain` filter (e.g., "industry = Technology")
   - Member capacity (`assignment_max`)
   - Lead created within `creation_delta_days` (default 7)
3. **Member assignment:** `_assign_and_convert_leads()` distributes to team members based on:
   - **Round-robin:** Even distribution
   - **Load-based:** Member with fewest leads gets next lead
   - **Preferred domain:** Member's `assignment_domain_preferred` is checked first
   - **Quota enforcement:** `assignment_max / 30 = daily quota`
4. **Duplicate merge:** Before assigning, duplicates are merged using `CRM_LEAD_FIELDS_TO_MERGE` (22 fields considered)
5. **Conversion:** Leads are auto-converted to opportunities if team doesn't use leads mode

**RERP's gap:** No assignment mechanism exists. Leads would need manual assignment until this is built.

---

## 4. Pipeline & Stage Model

### 4.1 Stage Entity

**RERP:** No Stage entity. No `stage_id` field on any entity.

**Odoo:** Complete stage model with:

| Field | Type | Purpose |
|-------|------|---------|
| `name` | Char | Stage name (e.g., "New", "Qualified", "Won") |
| `sequence` | Integer | Order position (lower = earlier) |
| `is_won` | Boolean | Marks the final "Won" stage |
| `rotting_threshold_days` | Integer | Days before stage shows as stale |
| `requirements` | Text | Internal requirements tooltip |
| `team_ids` | Many2many crm.team | Team association |
| `fold` | Boolean | Folded in Kanban when empty |
| `color` | Integer | Kanban color |
| `team_count` | Integer (compute) | Number of teams |

**Default stages (seeded in data):**
1. New (sequence 1, color 11)
2. Qualified (sequence 2, color 5)
3. Proposition (sequence 3, color 8)
4. Won (sequence 70, color 10, is_won=True)

### 4.2 Stage Behavior

When `is_won` changes on a stage, Odoo automatically:
- Sets all leads in that stage to `probability = 100`
- Sets `won_status = 'won'`
- Sets `date_closed = now()`
- If `is_won` is removed, recomputes automated probability

**Rotting:** `rotting_threshold_days` highlights stages that haven't been updated in N days. This is visual (Kanban color) but not yet an API endpoint.

### 4.3 Stage History & Tracking

| Field | Computation | Purpose |
|-------|-------------|---------|
| `date_last_stage_update` | Computed on write | Last time stage was changed |
| `stage_id_color` | Related to stage | Kanban color |
| `stage_percent` | Computed | Stage progress (for pipeline view) |
| `day_close` | Computed | `date_closed - date_open` |
| `day_open` | Computed | `date_open - create_date` |
| `date_conversion` | Computed | When lead was converted to opportunity |

---

## 5. Predictive Lead Scoring (Bayesian PLS)

This is one of Odoo's most sophisticated features — and it's completely absent in RERP.

### 5.1 Scoring Frequency Model

```python
class CrmLeadScoringFrequency(models.Model):
    _name = 'crm.lead.scoring.frequency'
    
    variable    — Field name (e.g., "email_state")
    value       — Field value (e.g., "correct", "incorrect")
    won_count   — How many times this combo led to a won deal
    lost_count  — How many times this combo led to a lost deal
    team_id     — Optional team scoping
```

Each row represents: "For team X, when field Y has value Z, it was won N times and lost M times."

### 5.2 Configurable Scoring Fields

Seven fields are configured for scoring (via `crm_pls_fields` config parameter):
- `phone_state` — Phone quality (correct/incorrect)
- `email_state` — Email quality (correct/incorrect)
- `state_id` — Country
- `country_id` — Country (duplicate?)
- `source_id` — Lead source
- `lang_id` — Language
- `tag_ids` — Classification tags

### 5.3 Scoring Algorithm (Bayesian)

The scoring is a simplified Bayesian inference:

```
P(Won | Field=Value) = P(Field=Value | Won) × P(Won) / P(Field=Value)

Where:
  P(Field=Value | Won) = won_count / total_won
  P(Field=Value | Lost) = lost_count / total_lost
  P(Field=Value) = (won_count + lost_count) / total_records
  
  P(Won) = total_won / (total_won + total_lost)
  
  P(Won | all_fields) = product of all individual P(Won|field=value)
                        × P(Won) / product of all P(field=value)
```

This is a **naive Bayes** classifier — it assumes field values are independent (which they're not, but it works well enough).

### 5.4 Scoring in the Lead Model

| Field | Type | Purpose |
|-------|------|---------|
| `automated_probability` | Float | Computed Bayesian score |
| `is_automated_probability` | Boolean | True if automated, false if manually overridden |
| `probability` | Float 0-100 | Final probability (manual or automated) |

### 5.5 Cron Job

```xml
<record id="website_crm_score_cron" model="ir.cron">
  <field name="name">Predictive Lead Scoring: Recompute</field>
  <field name="code">model._cron_update_automated_probabilities()</field>
  <field name="interval_number">1</field>
  <field name="interval_type">days</field>
  <field name="active" eval="False"/>  <!-- Disabled by default -->
</record>
```

Cron runs daily (disabled by default). Computes scores for all leads, updating `automated_probability` on each.

### 5.6 RERP's Opportunity

RERP has **zero scoring infrastructure**. No frequency tables, no cron job, no automated probability field. This is a potential differentiator for RERP: a Rust-based Bayesian classifier would be orders of magnitude faster than Odoo's Python ORM implementation.

---

## 6. Recurring Revenue Model

### 6.1 Recurring Plan Entity

**RERP:** No recurring revenue entity.

**Odoo:** Simple entity:

```python
class CrmRecurringPlan(models.Model):
    _name = 'crm.recurring.plan'
    
    name              — Plan name (e.g., "Annual", "Monthly", "Bi-annual")
    number_of_months  — Duration (e.g., 12, 24, 36)
    active            — Is this plan active
    sequence          — Display order
```

### 6.2 Recurring Fields on crm.lead

| Field | Type | Computation |
|-------|------|-------------|
| `recurring_plan` | Many2one crm.recurring.plan | Link to plan |
| `recurring_revenue` | Monetary | Manual entry |
| `recurring_revenue_monthly` | Monetary | `recurring_revenue / number_of_months` |
| `recurring_revenue_monthly_prorated` | Monetary | `MRR × probability / 100` |
| `recurring_revenue_prorated` | Monetary | `total_revenue × probability / 100` |

### 6.3 Prorated Revenue

```python
prorated_revenue = expected_revenue * probability / 100
recurring_revenue_monthly = recurring_revenue / plan.number_of_months
recurring_revenue_monthly_prorated = recurring_revenue_monthly * probability / 100
```

All computed, all stored for query performance. This is the foundation of MRR/ARR tracking.

---

## 7. Lost/Loss Management

### 7.1 Lost Reason Entity

**RERP:** No lost reason entity.

**Odoo:** Seed data with common reasons:
- "Too expensive"
- "We don't have people/skills"
- "Not enough stock"

| Field | Type | Purpose |
|-------|------|---------|
| `name` | Char | Reason description |
| `active` | Boolean | Soft delete |
| `leads_count` | Integer (compute) | How many leads lost for this reason |

### 7.2 Won/Lost on crm.lead

| Field | Type | Purpose |
|-------|------|---------|
| `won_status` | Selection (won/lost/pending) | Computed from stage |
| `lost_reason_id` | Many2one crm.lost.reason | Specific loss cause |

When a lead enters a stage marked `is_won=True`:
- `won_status = 'won'`
- `probability = 100`
- `date_closed = now()`

When a lead is marked lost (via wizard):
- `won_status = 'lost'`
- `lost_reason_id` is required
- Lead becomes inactive

---

## 8. Partner/Contact Synchronization

### 8.1 Bidirectional Sync

Odoo's `crm.lead` doesn't store contact details independently — it syncs with `res.partner`:

```python
PARTNER_FIELDS_TO_SYNC = [
    'lang', 'phone', 'function', 'website'  # Partial sync
]
PARTNER_ADDRESS_FIELDS_TO_SYNC = [
    'street', 'street2', 'city', 'zip', 'state_id', 'country_id'  # All-or-nothing
]
```

When a lead's partner is set:
1. Lead fields (email, phone, address, function) sync to the partner record
2. If the partner record is updated, the lead picks up new values (on read)
3. Address fields sync only if ALL address fields are set (to avoid mixed data)

### 8.2 Merge Fields

```python
CRM_LEAD_FIELDS_TO_MERGE = [
    'campaign_id', 'medium_id', 'source_id',    # UTM
    'email_cc',                                 # Mail
    'name', 'user_id', 'color', 'company_id',   # Core
    'lang_id', 'team_id', 'referred',           # Context
    'stage_id',                                 # Pipeline
    'expected_revenue', 'recurring_plan',       # Revenue
    'recurring_revenue',                        # Revenue
    'create_date', 'date_automation_last',      # Dates
    'date_deadline',                            # Dates
    'partner_id', 'title', 'partner_name',      # Partner
    'contact_name', 'email_from', 'function',   # Contact
    'phone', 'website',                         # Contact
]
```

When merging duplicate leads, these 22 fields are combined intelligently (non-blank values take precedence).

---

## 9. Wizards (Business Operations)

### 9.1 Lead-to-Opportunity Conversion

**RERP:** No conversion mechanism.

**Odoo:** Three wizards:
- `crm_lead_to_opportunity` — Single lead conversion
- `crm_lead_to_opportunity_mass` — Mass conversion
- `crm_merge_opportunities` — Merge duplicate opportunities

The wizard:
1. Creates a `res.partner` from the lead's contact data
2. Copies the lead to a new record with `type = 'opportunity'`
3. Links `partner_id` to the created partner
4. Copies UTM, team, revenue, and date fields
5. Removes the lead (or marks inactive)

### 9.2 Lost Reason Wizard

**RERP:** No lost reason handling.

**Odoo:** `crm_lead_lost` wizard — forces user to select a reason when marking a deal lost. This ensures data quality (no anonymous lost deals).

### 9.3 PLS (Predictive Lead Scoring) Update Wizard

**RERP:** No PLS update mechanism.

**Odoo:** `crm_lead_pls_update` wizard — manual trigger to recompute all lead probabilities. Called by cron but also available for manual recompute.

---

## 10. Security & Permissions

### 10.1 Odoo's Permission Model

**7 permission groups:**
- `group_use_lead` — Show lead menu (lead qualification mode)
- `group_use_recurring_revenues` — Show recurring revenues
- `group_sale_salesman` — Personal leads only
- `group_sale_salesman_all_leads` — All leads (pipeline)
- `group_sale_manager` — Full access including stages, scoring

**8 ir.rule records** for record-level access control:
- `Personal Leads` — Salesman sees only their own leads
- `All Leads` — Manager sees all leads
- `Multi-Company` — CRM leads filtered by company
- `Personal Activities` — Salesman sees only their activities
- `All Activities` — Manager sees all activities
- `Multi-Company Activities` — Activity filtering by company
- `Lead Plan` — Manager-only access to activity plans
- `Lead Plan Templates` — Manager-only access to plan templates

**29 access control records** — Each model has granular per-group permissions (read/write/create/unlink).

### 10.2 RERP's Gap

RERP has **no security configuration**. No groups, no rules, no permissions. All endpoints are accessible to anyone with API access. This is a production blocker.

---

## 11. UTM (Campaign/Medium/Source) Tracking

### 11.1 Odoo's UTM Mixin

```python
class UtmMixin:
    campaign_id  — Which campaign generated this lead
    medium_id    — Which medium (email, social, referral)
    source_id    — Which source (LinkedIn, Google, referral)
```

These are on every `crm.lead` via mixin. They link to `utm.campaign`, `utm.medium`, `utm.source` models.

### 11.2 RERP's Gap

RERP's specs don't include UTM fields. Leads have no campaign tracking, no source attribution. This is critical for marketing ROI measurement.

---

## 12. Activity & Communication Tracking

### 12.1 Odoo's Activity Mixins

```python
_mail.thread.cc       — Carbon copy tracking on emails
_mail.thread.blacklist — GDPR opt-out management
_mail.thread.phone    — Phone number validation
_mail.activity.mixin  — Follow-up activities (meetings, calls, tasks)
_mail.tracking.duration.mixin — Time spent in each stage
```

The `mail.activity` system allows:
- Scheduling follow-ups on leads
- Setting deadlines for activities
- Tracking which activities are completed/overdue
- Planning recurring follow-up sequences

### 12.2 Calendar Integration

```python
calendar_event_ids = One2many('calendar.event', 'opportunity_id')
```

Every meeting/phone call is a `calendar.event` linked to the lead/opportunity. Odoo's calendar view shows all activities across the pipeline.

### 12.3 RERP's Status

RERP has a `livechat` service with `Chat`, `Message`, and `Agent` entities. But:
- No email tracking
- No calendar integration
- No activity planning
- No communication history per entity
- No mail thread (email CC, opt-out, phone validation)

---

## 13. Data Quality & Validation

### 13.1 Odoo's Built-in Validation

| Feature | Implementation |
|---------|---------------|
| Email normalization | `email_normalized` — lowercase, dot removal |
| Email validation | `email_state` — correct/incorrect via regex |
| Email domain dedup | `email_domain_criterion` — btree index for dedup |
| Phone sanitization | `phone_sanitized` — E.164 format, btree indexed |
| Phone validation | `phone_state` — correct/incorrect via libphonenumber |
| Address formatting | `format.address.mixin` — standardized formats |
| Company enforcement | `check_company=True` on Many2one fields |
| Trigram indexing | `index='trigram'` on name, email, contact_name for fuzzy search |

### 13.2 RERP's Gap

Zero validation. No email normalization, no phone validation, no address formatting. This will lead to dirty data at scale.

---

## 14. Summary: What Makes Odoo a "World-Class" CRM

The depth is staggering. Here's what Odoo ships that RERP needs to build:

### Core Data Model (70+ fields on crm.lead)
- Full identity, contact, address model
- Team and salesperson assignment
- Pipeline stages with probability and won/lost
- Revenue, MRR, recurring plans
- Email/phone validation
- UTM campaign tracking
- Activity planning
- Partner synchronization
- Custom polymorphic properties

### Business Logic (3,818 lines across 6 model files)
- **Lead assignment** — Load-based, domain-based, round-robin with quota
- **Probability scoring** — Bayesian PLS with frequency tables
- **Stage management** — Won/lost, rotting thresholds, requirements
- **Partner sync** — Bidirectional sync between leads and contacts
- **Duplicate detection** — 22-field merge strategy
- **Lost reason tracking** — Mandatory reason on lost deals
- **Recurring revenue** — MRR/ARR computation with prorated values
- **Activity tracking** — Follow-ups, meetings, calendar events

### Data Quality
- Email normalization and validation
- Phone sanitization and validation (libphonenumber)
- Address standardization
- Trigram search indexing
- B-tree indexes on frequently queried fields

### Security
- 7 permission groups
- 8 record-level access rules
- 29 model-level access control entries
- Multi-company support

### Wizards
- Lead-to-opportunity conversion
- Mass conversion
- Opportunity merging
- Lost reason entry
- Scoring recompute

### Configuration
- 7 configurable scoring fields
- Cron job for PLS recomputation
- Default stage seed data
- Default lost reasons

---

## 15. RERP's Path to Parity

### Phase 1: Core Schemas (Weeks 1-4)
**Goal:** Define the data model. No business logic yet.

```yaml
# RERP Lead schema (example)
Lead:
  type: object
  required: [name, type]
  properties:
    name:
      type: string
      description: Opportunity/lead title
    type:
      type: string
      enum: [lead, opportunity]
    email_from:
      type: string
      format: email
    phone:
      type: string
    user_id:
      type: string
      format: uuid
    team_id:
      type: string
      format: uuid
    stage_id:
      type: string
      format: uuid
    expected_revenue:
      type: number
      format: float
    probability:
      type: number
      format: float
      minimum: 0
      maximum: 100
    date_deadline:
      type: string
      format: date
    campaign_id:
      type: string
      format: uuid
    medium_id:
      type: string
      format: uuid
    source_id:
      type: string
      format: uuid
    # ... 70+ more fields
```

### Phase 2: Team & Assignment (Weeks 5-8)
**Goal:** Make leads flow to the right people.

- Team entity with alias and assignment config
- TeamMember with load-based routing
- Cron endpoint for automatic assignment
- Lead conversion endpoint

### Phase 3: Pipeline & Revenue (Weeks 9-12)
**Goal:** Make the pipeline visible and quantifiable.

- Stage entity with sequence, probability, is_won
- RecurringPlan entity
- Revenue computation endpoints
- Lost reason entity

### Phase 4: Scoring & Intelligence (Weeks 13-20)
**Goal:** Add predictive features.

- ScoringFrequency entity
- Automated probability computation endpoint
- Batch scoring endpoint

### Phase 5: Security & Quality (Weeks 21+)
**Goal:** Production-ready.

- Permission model
- Email/phone validation
- Activity tracking
- UTM tracking
