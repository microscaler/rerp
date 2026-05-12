# Sales Team Management

> **Component:** Teams, members, territories, assignment, quotas, and organizational structure
> **Priority:** P1 — Organizations need teams, not solo users
> **Odoo Reference:** crm.team (759 lines), crm.team.member (99 lines), assignment algorithms

---

## The Pitch

**Buyer Question:** *How do I organize my sales organization, distribute work fairly, and hold people accountable?*

A CRM without team management is a personal contact book. Sales teams need territories, queues, auto-assignment, quotas, activity tracking, and performance dashboards. This component covers how sales organizations are modeled, how work is distributed, and how individual and team performance is measured.

---

## What This Component Does

1. **Team Structure** — Define teams (Inside Sales, Enterprise, SMB, Channel)
2. **Member Management** — Assign users to teams with capacities
3. **Auto-Assignment** — Distribute incoming leads fairly based on capacity, domain, and round-robin
4. **Domain-Based Routing** — Route leads by industry, region, or source to the right team
5. **Quota Management** — Set targets per rep, per team, per period
6. **Performance Tracking** — Track activities, deals closed, revenue per rep
7. **Work Queues** — Unassigned leads visible to team members
8. **Permission Model** — Owner vs team vs org visibility
9. **Alias/Email-to-Lead** — Team email aliases (e.g., sales@company.com) auto-create leads

---

## Entity Model

### Team Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Team name (e.g., "Enterprise Sales", "Inside Sales") |
| `description` | Text | No | Team description |
| `alias_id` | Foreign Key: mail.alias | No | Email alias for incoming emails (e.g., sales@company.com → leads) |
| `alias_name` | String (128) | No | Email alias (e.g., "info", "sales") |
| `alias_defaults` | JSON | No | Default values for leads created from emails |
| `use_leads` | Boolean | No | Enable lead qualification mode (stages: New → Qualified → Won) |
| `use_opportunities` | Boolean | No | Enable pipeline mode (stages: Qualification → Proposal → Won) |
| `assignment_enabled` | Boolean | Yes | Auto-assignment active for this team |
| `assignment_auto_enabled` | Boolean | No | Cron auto-assignment active |
| `assignment_optout` | Boolean | No | Skip auto-assignment for this team |
| `assignment_max` | Integer | Computed | Monthly capacity = Σ(member.assignment_max) |
| `assignment_domain` | String (64) | No | Domain filter for lead routing (e.g., "industry=Technology") |
| `assignment_domain_preferred` | String (64) | No | Preferred domain (higher priority routing) |
| `emoji` | String (16) | No | Team emoji for UI |
| `privacy_security` | Enum: [MY_TEAM, MY_COMPANY, ALL] | No | Default record visibility |

### Team Member Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | Foreign Key: User | Yes | Assigned salesperson |
| `crm_team_id` | Foreign Key: Team | Yes | Parent team |
| `assignment_enabled` | Boolean | Yes | Member can receive leads |
| `assignment_optout` | Boolean | No | Pause assignments (vacation, sabbatical) |
| `assignment_domain` | String (64) | No | Domain filter for this member |
| `assignment_domain_preferred` | String (64) | No | Preferred domain (checked first) |
| `assignment_max` | Integer | Yes | Average leads capacity (30 days) |
| `lead_day_count` | Integer | Computed | Leads assigned in last 24 hours |
| `lead_month_count` | Integer | Computed | Leads assigned in last 30 days |
| `quota_monthly` | Decimal(15,2) | No | Monthly revenue quota |
| `quota_quarterly` | Decimal(15,2) | No | Quarterly revenue quota |
| `quota_yearly` | Decimal(15,2) | No | Yearly revenue quota |
| `color` | Integer | No | Team color (1-16) for UI |

### Lead Assignment Tracking

Added to Lead/Opportunity entity:

| Field | Type | Purpose |
|-------|------|---------|
| `team_id` | Foreign Key: Team | Sales team ownership |
| `user_id` | Foreign Key: User | Assigned salesperson |
| `date_open` | DateTime | When lead was assigned (auto-set) |
| `day_open` | Float | Days from create to assign |
| `activity_summary` | Text | Last activity summary |

---

## Assignment Algorithm

This is the most sophisticated piece of business logic in Odoo CRM. Here's the full flow:

### Step 1: Team Allocation (which team?)

```
Cron: _cron_assign_leads() — runs daily
1. Find unassigned leads (team_id = NULL)
2. Filter: leads created within creation_delta_days (default: 7)
3. Match leads to teams based on:
   a. team.assignment_domain — e.g., "industry=Technology"
   b. team.assignment_domain_preferred — higher priority
   c. team.assignment_max (total team capacity)
4. For matching teams:
   a. Check if team has available capacity (team.assignment_max - team.used_count > 0)
   b. Assign lead to team
```

### Step 2: Member Assignment (which person?)

```
Cron: _assign_and_convert_leads() — runs after team allocation
1. Find team-assigned but unassigned leads (team_id set, user_id = NULL)
2. For each team:
   a. Get all active team members (assignment_enabled = true)
   b. For each member:
      - Check assignment_domain_preferred first
      - Check assignment_domain
      - Check capacity: lead_month_count < assignment_max / 30 (daily quota)
   c. Round-robin: pick member with fewest leads
   d. Assign lead to member
```

### Step 3: Duplicate Merge

Before assigning, check if lead is a duplicate of an existing lead. If so, merge using CRM_LEAD_FIELDS_TO_MERGE (22 fields: name, email, phone, company, etc.) and assign the merged result to the existing lead's owner.

### Step 4: Conversion

If team uses `use_opportunities=true` and not `use_leads=true`, auto-convert leads to opportunities.

---

## Required API Endpoints

### Team Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/teams` | List all teams |
| `GET` | `/teams/{id}` | Get team detail with members and stats |
| `POST` | `/teams` | Create team |
| `PATCH` | `/teams/{id}` | Update team |
| `DELETE` | `/teams/{id}` | Delete team (if no members/leads) |

### Team Member Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/teams/{id}/members` | List team members |
| `POST` | `/teams/{id}/members` | Add member to team |
| `PATCH` | `/teams/{id}/members/{member_id}` | Update member (quota, opt-out) |
| `DELETE` | `/teams/{id}/members/{member_id}` | Remove member |

### Assignment

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/assign/run` | Manually trigger assignment |
| `GET` | `/assign/status` | Check assignment status |
| `GET` | `/queues/unassigned` | List unassigned leads |
| `POST` | `/leads/{id}/assign` | Manually assign lead to user/team |
| `POST` | `/leads/{id}/transfer` | Transfer ownership |

### Performance

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/performance/rep/{id}` | Rep performance: leads, deals, revenue |
| `GET` | `/performance/team/{id}` | Team performance summary |
| `GET` | `/leaderboard` | Ranked list of reps by revenue/closes |
| `GET` | `/queues/team/{id}` | Unassigned leads for a team |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Capacity-Based Assignment
Odoo's `assignment_max` is a 30-day capacity. The daily quota is `assignment_max / 30`. The system tracks `lead_month_count` and only assigns when `lead_month_count < assignment_max / 30`. This prevents overloading busy reps.

**Recommendation: RERP should implement capacity-based assignment with daily quota tracking.**

### Pattern 2: Team Privacy Settings
`privacy_security` controls record visibility: `MY_TEAM` (team members only), `MY_COMPANY` (whole company), `ALL` (everyone). This maps to Odoo's access control levels.

**Recommendation: RERP should define team privacy as a core concept in the permission model.**

### Pattern 3: Alias for Email-to-Lead
`mail.alias` creates an email address that automatically creates leads when emails arrive. `alias_defaults` provides default values (team_id, user_id, stage_id).

**Recommendation: RERP should support team email aliases that auto-create leads with default team/user assignment.**

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based assignment performance** — Auto-assignment across 100,000+ leads with domain-based filtering is instantaneous. Python (Odoo) takes seconds at scale.
- **API-first team management** — Teams and members are OpenAPI-defined. Every client gets the same model.
- **Self-hosted org structure** — No vendor-defined hierarchy.

### Where RERP Lags
- **No team entity defined** — Zero team or member entities in any spec.
- **No permission model** — No roles, profiles, or field-level access.
- **No quota tracking** — No performance management.

---

## Competitive Intelligence Deep Dive

### Salesforce: Roles and Territories
**Roles and Territories** define the org hierarchy. **Sharing Rules** and **Permission Sets** control record-level access. **Assignment Rules** route leads based on criteria. **Queues** distribute work to groups. **Territory Management** handles complex B2B territories. **Quotas** at every org level with forecast rollups. **Omnichannel Routing** distributes leads to the best-available rep.

### Microsoft: Security Roles and Teams
**Security Roles** define permissions at entity, field, and record level. **Business Units** create org hierarchy. **Team Types** include Functional, Hierarchical, and Smart Groups. **Teams** integration lets reps collaborate on deals in real-time. **Outlook Plugin** logs all emails automatically.

---


### ServiceNow: Territory & Activity-Driven Teams
ServiceNow provides Sales Territory Management (geographic, product, or industry-based segments), Activity Management (plan/execute/tracks emails, meetings, calls, demos), and Team Management within Opportunity records. Sales agents are defined by their **fulfiller role** (not salesperson role) — this reflects ServiceNow's DNA as a service platform. Territory coverage is managed through automated assignment rules in Flow Designer. **Unique feature:** Activity Management is integrated with the broader service workflow — a sales meeting can trigger IT provisioning, HR onboarding, or field service dispatch as part of the same AI-orchestrated flow. **Gap vs. Salesforce:** No Role Hierarchy, no Territory Management 2.0, no Shared Activities across orgs. **Gap vs. Pipedrive:** No Activities Dashboard per rep with call/email/meeting tracking widgets. **Gap vs. Microsoft:** No Dynamics Team Insights with LinkedIn org changes.
## Implementation Roadmap

### Phase 1: Core Teams (1-2 weeks) — P1
1. Define `Team` entity: id, name, description, alias_id, use_leads, use_opportunities, assignment_enabled, assignment_domain
2. Define `TeamMember` entity: id, user_id, team_id, assignment_domain, assignment_max, quota
3. Add `team_id` and `user_id` to Lead entity
4. Implement team auto-assignment endpoint
5. Implement team member lead count tracking

### Phase 2: Assignment Algorithm (2-3 weeks) — P1
1. Domain-based lead routing (assignment_domain filter)
2. Round-robin assignment algorithm
3. Lead day/month count tracking per member
4. Team member overload detection
5. Team alias email-to-lead endpoint

### Phase 3: Performance & Permissions (3-4 weeks) — P1
1. Role-based access control (owner, team, org visibility)
2. Quota management per team member
3. Performance dashboard endpoint
4. Territory management
5. Work queue endpoint (unassigned leads)

---

## Key Takeaway for Buyers

Team management bridges individual reps and the organization. A buyer needs to know: *Can I organize my 5-person team the same way I organize my 500-person team?* RERP's OpenAPI-defined entities and Rust-based performance scale from startup to enterprise. But the team and permission models must be built.
