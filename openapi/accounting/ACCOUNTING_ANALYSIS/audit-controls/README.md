# Audit Controls & Segregation

> **Component:** Approval policies, segregation of duties, signature requests, audit events, and control exceptions
> **Priority:** P3 — Enterprise governance feature; important for regulated industries and large organizations
> **Odoo Reference:** Audit trail in all models, approval workflows in Enterprise, group-based access control

---

## The Pitch

**Buyer Question:** *Can I enforce segregation of duties, require approvals for sensitive transactions, maintain a complete immutable audit trail, and detect control exceptions automatically?*\

Audit controls are the governance layer that ensures financial integrity. They answer: *Who authorized this entry? Can the person who creates a vendor also approve payment? Is every change traceable?* This component handles approval policies (who approves what, at what thresholds), segregation of duties rules (preventing conflicts of interest), signature authorization (dual-signature requirements), immutable audit events (every change is logged), and control exception detection (automated alerts when controls are breached).

---

## What This Component Does

Audit Controls are the compliance backbone of accounting. They handle:

1. **Approval Policies** — Define who approves transactions by type, amount, department, or vendor
2. **Segregation of Duties** — Prevent conflicts (e.g., same person can't create and approve a vendor)
3. **Signature Authorization** — Dual/multi-signature requirements for payments above thresholds
4. **Audit Event Logging** — Immutable log of every creation, modification, and deletion
5. **Control Exceptions** — Automated detection of policy violations and alerts
6. **Approval Workflows** — Multi-level approval chains with escalation and delegation

---

## Entity Model

### Approval Policy Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Policy name |
| `entity_type` | Enum: [JOURNAL_ENTRY, BILL, INVOICE, PAYMENT, BUDGET_TRANSFER, ACCOUNT_CHANGE] | Yes | Applies to |
| `amount_threshold` | Decimal (15,2) | No | Minimum amount requiring approval |
| `department_ids` | Many2Many: Department | No | Departments covered |
| `approver_ids` | Many2Many: User | Yes | Approvers |
| `approval_sequence` | Integer | No | Multi-level sequence |
| `auto_approve_below` | Boolean | No | Auto-approve below threshold? |
| `escalation_policy` | Text | No | Escalation rules |
| `active` | Boolean | Yes | Policy active? |

**Total fields: ~10.**

### Segregation of Duties Rule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Rule name |
| `forbidden_action_pair` | JSON | Yes | Pairs of actions that can't share a user |
| `enforcement` | Enum: [HARD_BLOCK, WARNING, LOG_ONLY] | Yes | Enforcement level |
| `description` | Text | No | Rule description |
| `active` | Boolean | Yes | Rule active? |

**Total fields: ~6.**

### Audit Event Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `entity_type` | String | Yes | Entity type (e.g., "account.move") |
| `entity_id` | String | Yes | Entity record ID |
| `event_type` | Enum: [CREATE, UPDATE, DELETE, POST, CANCEL, APPROVE, REJECT] | Yes | Event type |
| `user_id` | Foreign Key: User | Yes | User who performed action |
| `field_name` | String (128) | No | Field changed |
| `old_value` | JSON | No | Previous value |
| `new_value` | JSON | No | New value |
| `ip_address` | String (64) | No | User IP |
| `user_agent` | String (512) | No | Browser/client info |
| `timestamp` | DateTime | Computed | Event timestamp |
| `company_id` | Foreign Key: Company | Yes | Affected company |

**Total fields: ~12.**

### Signature Request Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `entity_type` | Enum: [PAYMENT, JOURNAL_ENTRY, CONTRACT] | Yes | Entity type |
| `entity_id` | String | Yes | Entity ID |
| `amount` | Decimal (15,2) | Yes | Amount requiring signature |
| `required_signatures` | Integer | Yes | Number of signatures needed |
| `signature_count` | Integer | Computed | Signatures obtained |
| `signer_ids` | Many2Many: User | Yes | Required signers |
| `status` | Enum: [PENDING, PARTIAL, SIGNED, REJECTED, EXPIRED] | Yes | Signature status |
| `created_at` | DateTime | Computed | Request creation time |
| `expires_at` | DateTime | No | Request expiry time |

**Total fields: ~10.**

### Control Exception Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `rule_id` | Foreign Key: SoD Rule | No | Rule that was violated |
| `policy_id` | Foreign Key: Approval Policy | No | Policy that was breached |
| `entity_type` | String | Yes | Entity type |
| `entity_id` | String | Yes | Entity ID |
| `description` | Text | Yes | Exception description |
| `severity` | Enum: [LOW, MEDIUM, HIGH, CRITICAL] | Yes | Severity level |
| `status` | Enum: [OPEN, INvestigating, RESOLVED, ACCEPTED] | Yes | Status |
| `assigned_to` | Foreign Key: User | No | Assigned investigator |
| `created_at` | DateTime | Computed | Detection time |

**Total fields: ~10.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/audit/events` | List audit events with filters |
| `GET` | `/audit/events/{entity_type}/{id}` | Get events for specific entity |
| `GET` | `/audit/events/export` | Export audit log (CSV) |
| `GET` | `/controls/approval-policies` | List approval policies |
| `POST` | `/controls/approval-policies` | Create policy |
| `GET` | `/controls/sod-rules` | List segregation rules |
| `POST` | `/controls/sod-rules` | Create rule |
| `GET` | `/controls/exceptions` | List control exceptions |
| `POST` | `/controls/signature-request` | Create signature request |
| `POST` | `/controls/signature-request/{id}/sign` | Sign a request |
| `GET` | `/controls/compliance-report` | Generate compliance report |

---

## Competitive Intelligence

**NetSuite:** Automated approval workflows by role and amount. Segregation of duties with role-based access. Audit trail for all transactions. Custom approval hierarchies. Two-step authorization for payments.

**SAP S/4HANA:** Robust authorization concept with role-based access. Segregation of duties analyzer. Audit trail via SAP GRC. Approval workflows with Fiori apps. Signature management for payments.

**Odoo:** Audit trail in all models (Enterprise). Approval workflows for purchases, sales, payments. Group-based access control. Custom fields for audit logging.

**QuickBooks Online:** Limited audit trail. Basic user permission management. No segregation of duties features. No approval workflows (except in Advanced).

**Sage Intacct:** Role-based access control with permission sets. Segregation of duties monitoring. Comprehensive audit trail. Approval workflows with escalation.

**Xero:** Limited audit features. Basic user permissions. No approval workflows. No segregation of duties.

**Zoho Books:** Approval workflows for bills and payments. User roles with permissions. Audit log for changes. Good for mid-market but limited at scale.

---

## Implementation Roadmap

### Phase 1: Audit Event Logging (2 weeks) — P3
1. Define `AuditEvent` entity with comprehensive change tracking
2. Implement audit event creation on all critical entities
3. Implement audit log query and export endpoints
4. Add IP address and user-agent tracking

### Phase 2: Approval & SoD (3 weeks) — P3
1. Define `ApprovalPolicy` and `SoDRule` entities
2. Implement approval enforcement on transaction creation/posting
3. Implement segregation of duties check on user action
4. Implement control exception detection and alerts

### Phase 3: Signature & Compliance (2 weeks) — P3
1. Define `SignatureRequest` entity
2. Implement multi-signature workflow
3. Implement compliance report generation
4. Add exception resolution workflow

---

## Key Takeaway for Buyers

Audit controls are the trust layer — they answer the auditor's questions before they're asked. A buyer should ask: *Can my system prove who did what, when, and why? Can it prevent unauthorized actions and detect policy violations automatically?* RERP's API-first model means every action can be logged, and every control can be enforced programmatically. The gap with SAP/NetSuite is the depth of compliance frameworks (SOX, Sarbanes-Oxley automation, GRC integration). But for organizations that want complete control over audit and governance with API access, RERP provides the foundation.

**The immediate priority: implement audit event logging on all financial entities. This is the foundation for every other control.**
