# Sales Team Management

> **Component:** Teams, members, territories, and organizational structure
> **Competitive Landscape:** Salesforce, Microsoft Dynamics, SAP, HubSpot, Zoho, Pipedrive

## Pitch

**The Question Every Buyer Asks:** *"How do I organize my sales organization, distribute work fairly, and hold people accountable?"*

A CRM without team management is a personal contact book. Sales teams need territories, queues, auto-assignment, quotas, activity tracking, and performance dashboards. This component covers how sales organizations are modeled, how work is distributed, and how individual and team performance is measured.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM | Pipedrive |
|---------|----------|----------|------------|------------------------|---------|---------|----------|-----------|
| Sales team entity | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team member entity | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team alias/email | Planned | ✅ (alias_id) | ✅ (Email-to-Lead) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Auto-assignment rules | Planned | ✅ (assignment_enabled) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Round-robin assignment | Planned | ✅ (round-robin implied) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Load-based assignment | Planned | ✅ (assignment_max, lead_day_count) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Domain-based routing | Planned | ✅ (assignment_domain) | ✅ (Assignment Rules) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team quota tracking | Planned | ❌ | ✅ (Quotas) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Team activity metrics | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team leader permissions | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Territory management | Planned | ❌ | ✅ (Territories) | ✅ | ✅ | ✅ | ✅ (Teams) | ✅ |
| Role-based access control | Planned | ❌ | ✅ (Profiles/Roles) | ✅ (Security Roles) | ✅ | ✅ (Teams) | ✅ (Roles) | ✅ (Users) |
| Activity assignment tracking | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team performance dashboard | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Leaderboard | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Work queue | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Team email forwarding | Planned | ✅ (alias_contact) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Multi-org support | Planned | ✅ (user_company_ids) | ✅ (Multi-Org) | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based assignment performance** — Auto-assignment across 100,000+ leads with domain-based filtering in Rust is instantaneous. Python-based assignment (Odoo, Salesforce) can take seconds to minutes at scale.
- **API-first team management** — Teams and members are OpenAPI-defined entities. Every client (web, mobile, CLI) gets the same team model automatically.
- **Self-hosted org structure** — No vendor-defined team hierarchy. Build the org structure that matches your business.

### Where RERP Lags
- **No team entity defined** — The spec has `leads`, `contacts`, and `opportunitys` but no teams or members. This is a fundamental gap.
- **No permission model** — No roles, no profiles, no field-level access control. Every user sees everything.
- **No quota tracking** — Without quotas, there's no performance management.

---

## Competitive Intelligence Deep Dive

### Salesforce (Enterprise Org Structure — $25–$330/user/month)
**Roles and Territories** define the org hierarchy. **Sharing Rules** and **Permission Sets** control record-level access. **Assignment Rules** route leads automatically based on geographic, industry, or score criteria. **Queues** distribute work to groups (e.g., "Inbound Sales"). **Territory Management** handles complex B2B territories with overlapping rules and automatic account assignment. **Quotas** are set at every org level (rep → manager → VP → SVP) with forecast rollups. **Forecasts** support collaborative forecasting where teams adjust predictions. **Einstein Activity Capture** auto-logs emails and calendar events per rep. Enterprise: **Omnichannel Routing** distributes livechat, phone, and email leads to the best-available rep.

### Microsoft Dynamics 365 (Microsoft Integration — $65–$200/user/month)
**Security Roles** define permissions at entity, field, and record level. **Business Units** create org hierarchy with data isolation. **Team Types** include Functional (shared work), Hierarchical (manager sees subordinates), and Smart Groups (AI-based). **Auto-assignment** integrates with Power Automate for complex routing rules. **Teams** integration lets reps collaborate on deals in real-time — share deal context in Teams channels. **Outlook Plugin** logs all emails and calendar events automatically. Best for Microsoft-centric organizations already using Teams for collaboration.

### HubSpot (Simple Teams — Free → $1,800+/month for Enterprise)
**Teams** are basic: assign members, set roles (Admin, Manager, Member). **Permissions** are tiered: Admin (full access), Manager (own team + subordinates), Member (own records only). **Round-robin assignment** is built-in and automatic. **Omnichannel routing** distributes leads from web forms, livechat, email, and phone to the next available rep. **Deal ownership** can be transferred with notification. **Activity timelines** show all interactions per contact. Simple but effective for SMB to mid-market. No territory management, no complex quota structures.

### Pipedrive (Minimal Teams — $15–$99/user/month)
**Users** and **Teams** are basic. No territories, no roles, no quotas. **Shared inboxes** and **activities** provide some collaboration. **Deal ownership** is single-owner (no shared deals). **Activity-Based Selling** enforces logging before stage progression. The design philosophy: "sales teams are small, keep it simple." For teams under 20: this works perfectly. Beyond that: organizations need role-based permissions and territory management that Pipedrive doesn't provide.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2 weeks)
1. Define `Team` entity: id, name, description, alias_id, use_leads, use_opportunities, assignment_enabled
2. Define `TeamMember` entity: id, team_id, user_id, assignment_domain, assignment_max
3. Add team_id to Lead/Opportunity entities
4. Implement team auto-assignment endpoint
5. Implement team member lead quota tracking

### Phase 2 (2-4 weeks)
1. Domain-based lead routing (assignment_domain filter)
2. Round-robin assignment algorithm
3. Lead day/month count tracking per member
4. Team member overload detection
5. Team alias email-to-lead endpoint

### Phase 3 (4-8 weeks)
1. Role-based access control (owner, team, org)
2. Quota management per team member
3. Performance dashboard endpoint
4. Territory management
5. Work queue endpoint (unassigned leads)

---

## Key Takeaway for Buyers

Team management is the bridge between individual sales reps and the organization. A buyer needs to know: *"Can I organize my 5-person team the same way I organize my 500-person team?"* RERP's answer — through OpenAPI-defined entities and Rust-based performance — scales from startup to enterprise. But the team and permission models need to be built. Without them, RERP is a solo tool, not an organizational one.
