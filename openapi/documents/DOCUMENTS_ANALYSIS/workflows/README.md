# Review & Approval Workflows

> **Component:** Human-in-the-loop review queues, confidence-based routing, approval chains, batch review, and SLA tracking
> **Priority:** P2 — Human oversight for uncertain extractions and compliance-critical documents
> **UiPath Reference:** UiPath Document Understanding human review stations, ABBYY verification station, Rossum human-in-the-loop

---

## The Pitch

**Buyer Question:** *When the AI isn't 100% sure, does the system route to a human reviewer automatically — and does it track who reviewed what, when, and whether the review met SLA targets?*

No document extraction system is 100% accurate. A buyer needs confidence that low-confidence extractions are routed to human reviewers, that reviewers can see exactly what the AI extracted and what it's uncertain about, that approval chains are configurable, and that the system tracks review performance. This component covers the human-in-the-loop workflow that bridges AI automation with human judgment.

---

## What This Component Does

1. **Extraction Review Queue** — Centralized queue of documents awaiting human review
2. **Confidence-Based Routing** — Auto-route documents below a confidence threshold to human review; high-confidence documents skip review
3. **Review Interface Support** — Define the fields reviewers see (AI-extracted values, original document, confidence scores, uncertainty markers)
4. **Approval Chains** — Configurable multi-level approval (e.g., AP clerk → AP manager → CFO for large invoices)
5. **Batch Review** — Batch-approve multiple documents with the same decision
6. **Review Annotations** — Reviewers can add comments, corrections, and change extracted values
7. **SLA Tracking** — Track time-to-review, escalation rules for overdue reviews
8. **Review Analytics** — Reviewer performance, accuracy, throughput, error rates
9. **Dispute Resolution** — Flagged reviews can be escalated to senior reviewers

---

## Entity Model

### ReviewQueue Entity

The central queue that holds documents awaiting human review:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Document awaiting review |
| `extraction_id` | Foreign Key: ExtractionResult | Yes | Extraction result to review |
| `priority` | Enum: [LOW, NORMAL, HIGH, URGENT] | No | Review priority |
| `status` | Enum: [PENDING, IN_PROGRESS, REVIEWED, ESCALATED, REJECTED] | No | Current queue status |
| `confidence_score` | Float (0-100) | Yes | Extraction confidence score |
| `routing_reason` | Enum: [LOW_CONFIDENCE, COMPLIANCE_REQUIRED, RANDOM_SAMPLE, MANUAL_ASSIGN] | No | Why this was routed for review |
| `created_at` | DateTime | Yes | When added to queue |
| `reviewed_at` | DateTime | No | When review was completed |
| `sla_deadline` | DateTime | No | Deadline for review (configured per policy) |
| `sla_breached` | Boolean | No | Whether SLA deadline was missed |

### ReviewTask Entity

A specific review task assigned to a user:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `queue_id` | Foreign Key: ReviewQueue | Yes | Parent queue entry |
| `reviewer_id` | Foreign Key: User | Yes | Assigned reviewer |
| `status` | Enum: [ASSIGNED, IN_PROGRESS, COMPLETED, ESCALATED, EXPIRED] | No | Task status |
| `assigned_at` | DateTime | Yes | When task was assigned |
| `started_at` | DateTime | No | When reviewer started working |
| `completed_at` | DateTime | No | When review was completed |
| `time_to_review_minutes` | Float | Computed | Time from assignment to completion |
| `action` | Enum: [APPROVE, REJECT, CORRECT, ESCALATE] | No | Reviewer's decision |
| `corrections` | JSON | No | Fields that were corrected by reviewer |
| `comments` | Text | No | Reviewer comments |

### ApprovalChain Entity

Defines a multi-level approval chain:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Chain name (e.g., "Invoice Approval") |
| `description` | Text | No | Chain description |
| `trigger_condition` | JSON | Yes | Condition that triggers this chain (e.g., invoice > $10K) |
| `is_active` | Boolean | Yes | Enable/disable chain |
| `created_at` | DateTime | Yes | When chain was created |

### ApprovalStep Entity

A single step within an approval chain:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `chain_id` | Foreign Key: ApprovalChain | Yes | Parent chain |
| `step_number` | Integer | Yes | Order in chain (1, 2, 3, ...) |
| `approver_type` | Enum: [ROLE, USER, GROUP, DYNAMIC] | Yes | How approver is determined |
| `approver_value` | String (255) | Yes | Role name, user ID, group ID, or dynamic expression |
| `is_required` | Boolean | Yes | Must approve, or is informational? |
| `sla_minutes` | Integer | No | SLA for this step in minutes |

### ReviewAnnotation Entity

Annotations added by reviewers:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `task_id` | Foreign Key: ReviewTask | Yes | Parent review task |
| `field_name` | String (128) | Yes | Field being annotated |
| `annotation_type` | Enum: [COMMENT, CORRECTION, HIGHLIGHT, FLAG] | Yes | Type of annotation |
| `value` | Text | No | Annotation content |
| `created_at` | DateTime | Yes | When annotation was added |

### ApprovalDecision Entity

Records the final approval decision:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `task_id` | Foreign Key: ReviewTask | Yes | Review task |
| `decision` | Enum: [APPROVED, REJECTED, CONDITIONAL] | Yes | Final decision |
| `approved_by` | Foreign Key: User | Yes | Who approved |
| `approved_at` | DateTime | Yes | When approved |
| `conditions` | Text | No | Any conditions attached to approval |
| `rejection_reason` | Text | No | Why rejected |

### ReviewSLA Entity

SLA configuration for review workflows:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | SLA name (e.g., "Invoice Review 4 Hours") |
| `document_type` | String (64) | No | Applies to document type (NULL = all) |
| `priority` | Enum: [LOW, NORMAL, HIGH, URGENT] | No | Applies to priority level |
| `sla_hours` | Integer | Yes | Hours before SLA breach |
| `escalation_role` | String (64) | No | Role to escalate to on breach |
| `is_active` | Boolean | Yes | Enable/disable SLA |

---

## Entity Relationships

```
ReviewQueue (central review queue)
  ├── ReviewTask (via queue_id)              ← individual review tasks
  ├── DocumentStore (via document_id)         ← document being reviewed
  └── ExtractionResult (via extraction_id)    ← extraction to review

ReviewTask
  ├── ReviewQueue (via queue_id)              ← parent queue
  └── User (via reviewer_id)                  ← assigned reviewer

ApprovalChain
  ├── ApprovalStep (via chain_id)             ← steps in chain
  └── ReviewTask (via trigger_condition)      ← triggers chain

ReviewAnnotation
  ├── ReviewTask (via task_id)                ← task being annotated
  └── [Dynamic] (via field_name)              ← field being annotated

ApprovalDecision
  ├── ReviewTask (via task_id)                ← task being decided
  └── User (via approved_by)                  ← who approved

ReviewSLA
  └── ReviewQueue (via policy filter)         ← applies to queue entries
```

---

## Required API Endpoints

### Review Queue

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/review/queue` | List documents awaiting review |
| `GET` | `/review/queue/{id}` | Get queue entry detail |
| `POST` | `/review/queue/{id}/assign` | Assign to a reviewer |
| `POST` | `/review/queue/{id}/escalate` | Escalate to senior reviewer |
| `GET` | `/review/queue/stats` | Queue statistics (pending, overdue, SLA) |

### Review Tasks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/review/tasks` | List tasks for current user |
| `GET` | `/review/tasks/{id}` | Get task detail with extraction data |
| `PATCH` | `/review/tasks/{id}/start` | Start review (mark as in-progress) |
| `POST` | `/review/tasks/{id}/approve` | Approve with optional conditions |
| `POST` | `/review/tasks/{id}/reject` | Reject with reason |
| `POST` | `/review/tasks/{id}/correct` | Correct extracted values |

### Batch Review

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/review/batch/approve` | Batch-approve multiple documents |
| `POST` | `/review/batch/reject` | Batch-reject multiple documents |
| `GET` | `/review/batch/similar` | Find similar documents for batch review |

### Approval Chains

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/approval/chains` | List all approval chains |
| `POST` | `/approval/chains` | Create approval chain |
| `PATCH` | `/approval/chains/{id}` | Update approval chain |
| `DELETE` | `/approval/chains/{id}` | Delete approval chain |
| `GET` | `/approval/chains/{id}/steps` | List steps in chain |
| `POST` | `/approval/chains/{id}/steps` | Add step to chain |

### Review Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/review/analytics/throughput` | Reviewer throughput (documents/hour) |
| `GET` | `/review/analytics/accuracy` | Reviewer accuracy (corrections made) |
| `GET` | `/review/analytics/sla-compliance` | SLA compliance rates |
| `GET` | `/review/analytics/bottlenecks` | Bottleneck analysis (where reviews stall) |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted review workflows** — No per-reviewer licensing. Unlimited reviewers on your infrastructure.
- **OpenAPI-defined approval chains** — Every chain, step, and rule is machine-readable. API clients can build custom review interfaces.
- **Rust-level queue management** — Processing 10,000 pending reviews in Rust is instantaneous.
- **Configurable confidence thresholds** — Set different thresholds per document type, per department, per reviewer.
- **Zero marginal cost at scale** — Processing 100K review queues costs the same as 10K.

### Where RERP Lags

- **No review interface** — No UI for human review (API is defined, UI is separate).
- **No approval chains** — No multi-level approval workflow engine.
- **No batch review** — No ability to batch-approve similar documents.
- **No SLA tracking** — No time-to-review metrics or escalation rules.
- **No reviewer analytics** — No throughput or accuracy tracking.

---

## Competitive Intelligence Deep Dive

### UiPath Document Understanding — Human-in-the-Loop

UiPath's Document Understanding provides structured human review stations. When AI confidence drops below a threshold, documents are automatically routed to human reviewers. The review interface shows the AI-extracted data alongside the original document, with highlighted uncertain fields. Reviewers can correct values with a click. UiPath tracks reviewer accuracy, time-to-review, and batch approval rates. The key differentiator is deep RPA integration — once approved, data flows automatically into downstream processes. Pricing: $0.18-$0.24/page for Document Understanding license.

### ABBYY FlexiCapture — Verification Station

ABBYY's verification station provides human review for extraction results. Reviewers see the original document with extracted fields overlaid. Uncertain extractions are highlighted for quick correction. ABBYY supports configurable review workflows with approval chains and escalation rules. The key differentiator is enterprise-grade compliance — audit trails for every review action, configurable retention, and multi-tenancy. Cost: part of $100K+ enterprise license.

### Rossum — Human-in-the-Loop AI

Rossum's key differentiator is its "zero training required" approach — the AI learns from the first document and improves through human corrections. Reviewers correct uncertain extractions, and Rossum's deep learning model updates automatically. No retraining pipeline needed. The review interface is clean and focused — one document at a time, with AI-extracted values clearly displayed. Reviewers can approve, reject, or correct. Rossum charges per-volume (custom quote).

---

## Implementation Roadmap

### Phase 1: Core Review Queue (2-3 weeks) — P2

1. Define `ReviewQueue` entity: id, document_id, extraction_id, priority, status, confidence_score, routing_reason
2. Define `ReviewTask` entity: id, queue_id, reviewer_id, status, assigned_at, completed_at, action, corrections
3. Implement queue entry creation on low-confidence extraction (trigger from extraction service)
4. Implement queue listing endpoint (GET /review/queue)
5. Implement task assignment endpoint (POST /review/queue/{id}/assign)
6. Implement review completion endpoints (approve/reject/correct)

### Phase 2: Confidence-Based Routing (2-3 weeks) — P2

1. Define confidence threshold configuration per document type
2. Implement auto-routing logic (extract confidence < threshold → queue)
3. Implement routing reason tracking (LOW_CONFIDENCE, COMPLIANCE_REQUIRED, etc.)
4. Add priority-based queue ordering
5. Implement reviewer assignment rules (round-robin, skill-based, workload-based)
6. Implement queue statistics endpoint

### Phase 3: Approval Chains (3-4 weeks) — P2

1. Define `ApprovalChain` and `ApprovalStep` entities
2. Implement chain creation and management endpoints
3. Implement chain execution engine (evaluate trigger conditions, create approval tasks)
4. Implement approval decision recording (APPROVED, REJECTED, CONDITIONAL)
5. Add configurable SLA per approval step
6. Implement escalation rules (auto-escalate on SLA breach)

### Phase 4: Batch Review & Analytics (2-3 weeks) — P2

1. Implement batch approve/reject endpoints
2. Implement similar document detection for batch review
3. Implement reviewer analytics (throughput, accuracy, SLA compliance)
4. Implement bottleneck analysis (where reviews stall)
5. Add review annotations (comments, corrections, highlights)

### Phase 5: Advanced Features (3-4 weeks) — P2

1. Implement multi-channel review (email, API, web UI)
2. Add review dispute resolution (flag for senior reviewer)
3. Implement review templates (pre-configured review actions per document type)
4. Add review audit trail (every action logged with timestamp and user)
5. Implement review performance dashboards

---

## Key Takeaway for Buyers

Review and approval workflows are where automation meets accountability. A buyer needs to know: *When the AI isn't sure, does the system route to a human — and can I track who reviewed what, when, and whether it met SLA targets?* RERP's advantage is self-hosted review workflows with zero per-reviewer licensing and configurable confidence thresholds per document type. The OpenAPI-defined approval chains mean every workflow is machine-readable and API-accessible. The immediate priority: define the ReviewQueue and ReviewTask entities, implement low-confidence auto-routing, and build the review completion endpoints. Everything else builds on this foundation.
