# Marketing Integration

> **Component:** Campaign tracking, web form capture, visitor analytics, and top-of-funnel automation
> **Priority:** P3 — Important but can be phased; API form capture first
> **Odoo Reference:** crm.utm.mixin, crm_iap_lead, website_crm, event_crm, survey_crm, mass_mailing_crm

---

## The Pitch

**Buyer Question:** *Can I capture leads from every channel, track where they came from, and nurture them before they reach sales?*

Marketing and CRM should not be two separate systems. A world-leading CRM integrates seamlessly with marketing channels: web forms, events, surveys, mass mailing, and social media. This component covers how leads are captured from the web, how campaign ROI is measured, and how marketing efforts flow directly into the sales pipeline. Without this, the CRM only sees what the sales team manually enters — the top of the funnel is invisible.

---

## What This Component Does

1. **Campaign Tracking** — UTM parameters (campaign, medium, source) on every lead for attribution
2. **Web Form Capture** — Public API endpoint that accepts form submissions from any website
3. **Visitor-to-Lead Conversion** — Track anonymous website visitors and convert them when they fill a form
4. **Event Registration** — Event attendees automatically become leads in CRM
5. **Survey-to-Lead** — Survey responses create CRM records with answers as lead properties
6. **Mass Mailing** — Send bulk emails from CRM with open/click tracking
7. **SMS Campaigns** — Text message campaigns linked to leads
8. **Lead Nurturing** — Automated drip campaigns for unqualified leads
9. **Landing Pages** — Hosted form pages with A/B testing
10. **Social Media Capture** — LinkedIn, Twitter, Facebook lead capture
11. **GDPR Consent** — Track consent on all form submissions
12. **Multi-Channel Attribution** — See which channels actually drive revenue

---

## Entity Model

### UTM Campaign Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Campaign name (e.g., "Q3 Webinar Series", "Google Ads Brand") |
| `active` | Boolean | No | Soft delete (default: true) |
| `sequence` | Integer | No | Display order |
| `company_id` | Foreign Key: Company | No | Multi-company support |

### UTM Medium Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (64) | Yes | Medium name (email, cpc, social, organic, referral, direct) |
| `active` | Boolean | No | Soft delete |

### UTM Source Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (64) | Yes | Source name (google, linkedin, facebook, twitter, newsletter, referral) |
| `medium_id` | Foreign Key: UTM_Medium | No | Associated medium type |
| `active` | Boolean | No | Soft delete |

### Web Form Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Form name (e.g., "Contact Us", "Request Demo") |
| `redirect_url` | String | No | Redirect URL after submission |
| `is_active` | Boolean | No | Enable/disable form |
| `capture_fields` | JSON | Yes | Form fields to capture (key-value schema) |
| `team_id` | Foreign Key: Team | No | Auto-assign to team |
| `user_id` | Foreign Key: User | No | Auto-assign to rep |
| `utm_source_id` | Foreign Key: UTM_Source | No | Default source for this form |
| `utm_medium_id` | Foreign Key: UTM_Medium | No | Default medium for this form |
| `utm_campaign_id` | Foreign Key: UTM_Campaign | No | Default campaign for this form |
| `consent_required` | Boolean | No | Require GDPR consent checkbox |
| `consent_text` | Text | No | Consent checkbox text |
| `api_endpoint` | String | Computed | Public endpoint: /forms/{id} |
| `embed_code` | String | Computed | JS embed snippet |

### Visitor Tracking Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `visitor_id` | String | Yes | Anonymous visitor ID (session cookie) |
| `url_visited` | String | Yes | Last URL visited |
| `referrer` | String | No | Page that referred them |
| `user_agent` | String | No | Browser/device info |
| `country` | String | No | Country from IP |
| `city` | String | No | City from IP |
| `first_seen` | DateTime | Yes | First visit timestamp |
| `last_seen` | DateTime | Yes | Last visit timestamp |
| `pages_viewed` | Integer | No | Total pages viewed |
| `converted_to_lead` | Boolean | No | Whether this visitor became a lead |
| `lead_id` | Foreign Key: Lead | No | Converted lead (if any) |

### Event Registration Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `event_name` | String (255) | Yes | Event name |
| `event_date` | DateTime | Yes | Event date/time |
| `attendee_name` | String (255) | Yes | Attendee name |
| `attendee_email` | String (255) | Yes | Attendee email |
| `attendee_company` | String (255) | No | Attendee company |
| `attendee_phone` | String (64) | No | Attendee phone |
| `attendee_function` | String (128) | No | Attendee job title |
| `lead_id` | Foreign Key: Lead | No | Created lead record |
| `source` | String (64) | No | Registration source (web, email, referral) |
| `attended` | Boolean | No | Did they actually attend? |
| `registration_date` | DateTime | Yes | When they registered |

### Survey Response Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `survey_name` | String (255) | Yes | Survey name |
| `respondent_email` | String (255) | Yes | Respondent email |
| `respondent_name` | String (255) | No | Respondent name |
| `responses` | JSON | Yes | All survey answers |
| `lead_id` | Foreign Key: Lead | No | Created lead (if applicable) |
| `response_date` | DateTime | Yes | When responded |

---

## Campaign Attribution Flow

```
User clicks URL with UTM params:
  https://example.com/pricing?utm_source=google&utm_medium=cpc&utm_campaign=q3_brand

1. Browser receives URL with utm_source, utm_medium, utm_campaign
2. Cookie/localStorage stores UTM values
3. User fills web form → form submission includes stored UTM values
4. Lead created with campaign_id, medium_id, source_id
5. When lead converts → opportunity tracks same attribution
6. When deal closes → revenue attributed to campaign
```

---

## Required API Endpoints

### UTM Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/utm/campaigns` | List campaigns |
| `POST` | `/utm/campaigns` | Create campaign |
| `GET` | `/utm/mediums` | List mediums |
| `POST` | `/utm/mediums` | Create medium |
| `GET` | `/utm/sources` | List sources |
| `POST` | `/utm/sources` | Create source |

### Web Form Capture (Public API)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/forms/{id}` | Public endpoint — submit form data, creates lead |
| `GET` | `/forms/{id}/config` | Get form configuration |
| `POST` | `/forms/{id}/submit` | Submit with validation and GDPR consent |

### Visitor Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/analytics/track` | Track a page view (anonymous) |
| `GET` | `/analytics/visitors` | List visitors with filtering |
| `GET` | `/analytics/visitors/{id}/journey` | Visitor journey history |
| `POST` | `/analytics/identify` | Identify anonymous visitor (email match) |

### Campaign Reporting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/campaigns/{id}/leads` | Leads generated by campaign |
| `GET` | `/campaigns/{id}/revenue` | Revenue attributed to campaign |
| `GET` | `/campaigns/{id}/roi` | Campaign cost vs revenue |
| `GET` | `/analytics/source-effectiveness` | Lead volume and conversion by source |

---

## Odoo Technical Patterns to Follow

### Pattern 1: UTM Mixin
Odoo uses `utm.mixin` to add `campaign_id`, `medium_id`, `source_id` to ANY model (crm.lead, sale.order, website_visitor). This means every touchpoint in the business is trackable, not just CRM records.

**Recommendation: RERP should define UTM fields in a reusable section of the OpenAPI spec that can be included in any entity schema.**

### Pattern 2: Visitor Tracking via Cookie
Odoo's website_crm tracks anonymous visitors with a session cookie. When the visitor fills a form (or is revealed via IAP), the visitor record is linked to the lead.

**Recommendation: RERP should track visitors via API calls with a visitor_id parameter. The public form endpoint accepts this ID and links the lead to the visitor.**

### Pattern 3: Event/Survey Integration as First-Class Entities
Odoo's event_crm and survey_crm create leads automatically when people register or respond. This is a plugin pattern — the core CRM doesn't know about events or surveys, but the plugins add the bridge.

**Recommendation: RERP should define event_registration and survey_response as entities that CREATE leads. The lead entity should accept optional event/survey IDs.**

---

## Competitive Positioning

### Where RERP Wins
- **API-first web form capture** — Any external site can POST a form directly to the RERP CRM API. No need to embed JavaScript widgets or use a third-party form builder.
- **Built-in event and survey modules** — Event registration and surveys should be first-class CRM citizens, not afterthoughts.
- **Self-hosted marketing stack** — No Marketing Cloud subscription. Everything runs on your infrastructure.
- **UTM tracking at entity level** — Every record can have campaign/medium/source tracking.

### Where RERP Lags
- **No web form widget** — HubSpot and Salesforce let you embed a form on any website with a simple `<script>` tag.
- **No landing page builder** — No drag-and-drop page creation.
- **No social media integration** — No LinkedIn, Twitter, Facebook lead capture.
- **No nurturing workflows** — No drip campaigns.

---

## Competitive Intelligence Deep Dive

### Salesforce Marketing Cloud ($2,500–$10,000+/month add-on)
**CloudPages** hosts landing pages with embedded forms and A/B testing. **Journey Builder** creates multi-channel nurture sequences. **Social Studio** captures leads from Twitter, LinkedIn, Facebook. **Email Studio** handles 10M+ emails/day. The scale is unmatched but pricing is enterprise-grade.

### HubSpot Marketing Hub ($20–$4,500+/month)
**Embedded Forms** work on any website via simple JS embed. **Landing Pages** with drag-and-drop and A/B testing. **Chatflows** convert visitors in real-time. **Lead Scoring** tracks engagement across page views, email opens, form fills. **Attribution Reporting** shows which campaigns drive revenue. The integration between marketing and CRM is seamless because they're the same product.

### Microsoft Dynamics (Microsoft Ecosystem)
**Microsoft Advertising** (Bing Ads, LinkedIn Ads) for closed-loop attribution. **LinkedIn Sales Navigator** for lead discovery. **Customer Insights** unifies marketing and sales data. **Power BI** creates marketing dashboards. Best for Microsoft-centric organizations.

### Zoho MarketingPlus ($25–$100/user/month)
**Zoho Forms** creates custom forms with conditional logic. **Zoho Campaigns** handles email marketing and A/B testing. **Zoho Social** manages social media across 10+ platforms. **Zoho Survey** captures lead data. Best value marketing integration.

### Pipedrive (No Native Marketing)
Pipedrive has **no native marketing features**. Connects to Zapier, Mailchimp, HubSpot, and 500+ apps. **Web Forms** are basic — create a form, share a link. No landing pages, no A/B testing. For simple sales teams using separate marketing tools, this works.

---

## Implementation Roadmap

### Phase 1: UTM Tracking (1-2 weeks)
1. Define `UTMCampaign`, `UTMMedium`, `UTMSource` entities
2. Add `campaign_id`, `medium_id`, `source_id` to Lead entity
3. Implement UTM parameter extraction from URL query string
4. Implement basic campaign reporting (leads per campaign)
5. Add UTM fields to Opportunity for revenue attribution

### Phase 2: Web Form Capture (2-3 weeks)
1. Define `WebForm` entity with capture_fields (JSON schema)
2. Implement public form endpoint (POST /forms/{id})
3. Add form validation and GDPR consent handling
4. Implement form-to-lead conversion
5. Add visitor tracking endpoint with visitor_id parameter

### Phase 3: Events & Surveys (2-3 weeks)
1. Define `EventRegistration` entity
2. Define `SurveyResponse` entity
3. Implement event registration to lead conversion
4. Implement survey response to lead conversion
5. Add basic event/survey reporting

### Phase 4: Advanced Marketing (3-4 weeks)
1. Mass mailing endpoint (SMTP API)
2. SMS campaign endpoint (Twilio integration)
3. Email nurture workflow engine
4. Social media lead capture (Twitter, LinkedIn, Facebook)
5. Marketing ROI reporting (campaign cost vs. revenue)

---

## Key Takeaway for Buyers

Marketing integration is where CRM meets the business's top of funnel. A buyer wants to know: *If I put a form on my website tomorrow, will leads flow into the CRM automatically?* RERP's API-first approach answers "yes" — any website can POST to the form endpoint. The tradeoff: no out-of-the-box JS widget like HubSpot, but also no vendor lock-in or JavaScript dependency. For developer-driven organizations, this is a feature. For marketers who want "click and deploy," it's a hurdle.
