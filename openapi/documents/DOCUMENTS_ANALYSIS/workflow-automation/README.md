# Workflow Automation

> **Component:** Automating document processing workflows with human-in-the-loop validation
> **Priority:** P2 — Valuable but non-blocking for initial adoption
> **DocuPipe Reference:** POST /workflow/on_submit_document (upload→classify→standardize in one call), webhooks for async callbacks
> **Hyperscience Reference:** Blocks and Flows composable architecture, Custom Code Blocks for business logic, AI-in-the-Loop orchestration

---

## The Pitch

**Buyer Question:** *Can I automate the entire document lifecycle — from ingestion through extraction, validation, and downstream integration — with human oversight for exceptions?*

If the answer is no, you have a document processing tool, not a workflow platform. The value of document intelligence is not just in extracting data — it's in acting on it. Automated routing, approval workflows, exception handling, and downstream integration are what transform extracted data into business outcomes. This component defines how documents flow through processing stages, how approvals are managed, and how exceptions are handled.

---

## What This Component Does

Workflow Automation is the orchestration layer:

1. **Visual Workflow Builder** — Drag-and-drop workflow definition
2. **Approval Workflows** — Multi-level approval chains with notifications
3. **Exception Handling** — Human review for low-confidence extractions
4. **Scheduled Workflows** — Time-based processing triggers
5. **Event-Driven Workflows** — Trigger workflows on document events
6. **Downstream Integration** — Send extracted data to ERP, accounting, CRM
7. **Parallel Processing** — Concurrent workflow branches
8. **Audit Trail** — Complete workflow execution history

---

## Entity Model

### Workflow Entity

The core workflow definition. Every workflow is a graph of nodes (stages) connected by edges (transitions).

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Workflow name (e.g., "Invoice Processing") |
| `definition` | JSONB | Yes | Workflow definition (nodes, edges, transitions) |
| `is_active` | Boolean | No | Workflow activation (default: true) |
| `document_types` | UUID[] | No | Applicable document types (empty = all) |
| `trigger_condition` | JSONB | No | Event trigger (document.created, extraction.rejected, etc.) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `created_by` | UUID | No | Creator |
| `updated_at` | DateTime | Yes | Last update |

### Workflow Execution Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `workflow_id` | Foreign Key: Workflow | Yes | Parent workflow |
| `document_id` | Foreign Key: Document | Yes | Processed document |
| `status` | Enum: [PENDING, RUNNING, COMPLETED, FAILED, CANCELLED] | Yes | Execution state |
| `current_node` | String (255) | Yes | Current workflow node |
| `started_at` | DateTime | No | Execution start |
| `completed_at` | DateTime | No | Execution completion |
| `duration_ms` | Float | No | Total execution time |
| `error_message` | Text | No | Failure details |

### Workflow Execution Step Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `execution_id` | Foreign Key: Workflow Execution | Yes | Parent execution |
| `node_name` | String (255) | Yes | Workflow node name |
| `status` | Enum: [PENDING, RUNNING, COMPLETED, FAILED, SKIPPED] | Yes | Step state |
| `started_at` | DateTime | No | Step start |
| `completed_at` | DateTime | No | Step completion |
| `duration_ms` | Float | No | Step duration |
| `output` | JSONB | No | Step output data |

### Approval Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `execution_id` | Foreign Key: Workflow Execution | Yes | Parent execution |
| `approver_id` | UUID | Yes | Approver user |
| `status` | Enum: [PENDING, APPROVED, REJECTED] | Yes | Approval state |
| `comments` | Text | No | Approval comments |
| `created_at` | DateTime | Yes | Creation timestamp |
| `completed_at` | DateTime | No | Approval timestamp |
| `delegated_to` | UUID | No | If delegated, who it was delegated to |

---

## Entity Relationships

```
Workflow (central definition)
  ├── Workflow Execution (one-to-many)    ← via workflow_id
  └── Workflow Step (one-to-many)         ← via execution_id (through Execution)

Workflow Execution
  ├── Workflow (many-to-one)              ← via workflow_id
  ├── Document (many-to-one)              ← via document_id
  ├── Workflow Step (one-to-many)         ← via execution_id
  └── Approval (one-to-many)              ← via execution_id

Workflow Step
  └── Workflow Execution (many-to-one)    ← via execution_id

Approval
  └── Workflow Execution (many-to-one)    ← via execution_id
```

---

## Hyperscience Technical Patterns to Follow

### Pattern 1: Blocks and Flows Architecture

Hyperscience's Hypercell platform uses a composable architecture built on two primitives:

- **Blocks** — Discrete processing functions: document ingestion, classification, extraction, validation, business rule enforcement, decisioning, LLM-powered analysis
- **Flows** — Sequences of Blocks connected and arranged into executable workflows

Blocks can be pre-built or custom (Python code). Custom Code Blocks let you embed business logic directly within workflows: "Build custom validation rules, implement decisioning logic, enrich extracted data, and trigger automated actions."

```
Flow example: Invoice Processing
┌─────────┐    ┌──────────┐    ┌────────────┐    ┌──────────┐
│ Ingest  │───▶│Classify  │───▶│Extract     │───▶│Approve   │
│ Document│    │  Type    │    │  Fields    │    │  (HITL)  │
└─────────┘    └──────────┘    └────────────┘    └──────────┘
       │              │               │              │
       ▼              ▼               ▼              ▼
    S3/GCS      Rules + ML      Schema match   Low confidence
    trigger     auto-route      routing        → human review
```

**Recommendation: RERP should implement a Blocks and Flows architecture.** Define pre-built Blocks for each processing stage (Ingest, Classify, Extract, Validate, Approve, Integrate). Users create Flows by connecting Blocks in sequence or parallel. Custom Blocks can be defined via Python or Rust code. This gives users the flexibility to customize workflows without code, while providing the extensibility for complex business logic.

### Pattern 2: AI-in-the-Loop Orchestration

Hyperscience's AI-in-the-Loop architecture progressively applies ML models to eliminate manual effort. Only the most complex exceptions reach a human. This creates a continuously optimizing workflow that maximizes automation.

```
Standard flow:
Document → ML Model → High confidence → Auto-approve
                    → Low confidence  → Human review
                                       → User corrects → Model retrains

Result: 98%+ automation rate with 99.5% accuracy
```

**Recommendation: RERP should implement AI-in-the-Loop.** Define confidence thresholds at each workflow stage. When confidence is above threshold, auto-approve. When below threshold, route to human review. When a user corrects the result, feed the correction back into the model for retraining. This creates a self-improving workflow that gets better over time.

### Pattern 3: API-First Integration as Blocks

Hyperscience treats integrations as configurable Blocks within the workflow engine. These integration Blocks enable data to flow effortlessly between systems, ensuring extracted, validated, and enriched information reaches the right destinations in real time.

**Recommendation: RERP should define Integration Blocks** for common downstream systems: accounting (RERP general-ledger), procurement (RERP purchase-order), CRM (RERP lead-contact). Each Integration Block has a configurable payload template that maps extracted fields to the target API.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Simple Workflow Triggers
DocuPipe supports workflow automation via `POST /workflow/on_submit_document` — upload → classify → standardize in a single API call. This is the simplest possible workflow: chain three operations into one HTTP call. Webhooks notify when each stage completes.

**DocuPipe workflow:**
1. `POST /workflow/on_submit_document` with document + schema + classify flag
2. System auto-classifies, extracts using schema, returns JSON result
3. Webhook fires on completion

**Key strengths:** Single API call chains multiple operations, webhooks for async notification
**Key weaknesses:** Linear workflow only, no branching, no approvals, no conditional logic

### Hyperscience: Blocks and Flows with Custom Code
Hyperscience's Hypercell platform uses a composable Blocks-and-Flows architecture. Pre-built Blocks for ingestion, classification, extraction, validation, decisioning, and LLM analysis. Custom Code Blocks allow Python code execution for business logic. AI-in-the-Loop progressively eliminates manual effort — only complex exceptions reach humans. Supports AWS, Google, Azure, on-premises, and FedRAMP High deployments.

**Key strengths:** Composable architecture, custom code blocks, AI-in-the-Loop, multi-deployment
**Key weaknesses:** Enterprise-only, no self-hosted option without significant cost

### Rossum: End-to-End Workflow with Master Data Matching
Rossum's workflow automation is enterprise-grade. Custom business logic, master data matching (cross-references extracted values with SAP, Coupa, Workday, Oracle), duplicate detection, and integrated intelligent mailbox. Workflow reporting tracks automation rates, team performance, and SLA compliance. 14-day free trial available.

**Key strengths:** Enterprise workflow engine, SAP/Workday integration, SLA tracking
**Key weaknesses:** Enterprise-only (~$18k+/yr), no self-hosted option

### Paperless-ngx: Simple Rule-Based Workflow
Paperless-ngx includes a workflow system for document ingestion. Documents can be automatically categorized, tagged, and routed based on rules. Email processing supports post-processing actions (mark as read, delete). Simple but effective for basic automation needs.

**Key strengths:** Free and self-hosted, simple rule-based automation
**Key weaknesses:** No approval chains, no downstream integration, no conditional branching

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, no workflow licensing** — Unlike Rossum (included in $18k+/yr) or Hyperscience (enterprise-only), RERP provides full workflow automation at zero cost
- **Blocks and Flows architecture** — Unlike DocuPipe (single API call only), RERP supports branching, approvals, and conditional logic
- **OpenAPI-defined workflows** — Every workflow node and transition is defined in OpenAPI specs, enabling automatic SDK generation

### Where RERP Lags
- **No workflow engine** — Not yet implemented
- **No approval chains** — Not yet implemented
- **No downstream integration** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Workflow Engine (4-6 weeks) — P2
1. Define `Workflow` entity with JSONB definition (nodes, edges, transitions)
2. Define `Workflow Execution` entity with status tracking
3. Define `Workflow Execution Step` entity for stage-by-stage tracking
4. Implement workflow execution engine (state machine runner)
5. Implement basic approval workflow with notifications
6. Implement exception handling for low-confidence extractions
7. Implement workflow audit trail

### Phase 2: Advanced Workflow Features (4-6 weeks) — P2
1. Implement visual workflow builder (drag-and-drop node editor)
2. Implement scheduled workflow triggers (cron-based)
3. Implement event-driven workflows (document events trigger flows)
4. Implement parallel workflow branches (split/join)
5. Implement workflow versioning and comparison
6. Implement conditional routing (if/else based on extraction confidence)

### Phase 3: Integration Layer (3-4 weeks) — P3
1. Define Integration Blocks for common downstream systems
2. Implement RERP general-ledger integration (accounting)
3. Implement RERP purchase-order integration (procurement)
4. Implement email notifications with templates
5. Implement webhook delivery for workflow events

### Phase 4: Intelligence & Optimization (3-4 weeks) — P3
1. Implement AI-in-the-Loop (auto-approve above confidence threshold)
2. Workflow performance analytics dashboard
3. Automated workflow optimization suggestions
4. SLA monitoring and reporting
5. Workflow template marketplace (pre-built workflows for common use cases)

---

## Key Takeaway for Buyers

RERP Documents' workflow pitch is **OpenAPI-first, self-hosted, and ERP-native**. Unlike Rossum (enterprise-only, ~$18k/year) or Hyperscience (custom Python code, enterprise pricing), RERP provides visual workflow automation at zero cost. Unlike Textract (AWS Step Functions, infrastructure-heavy), RERP's workflow engine is purpose-built for document processing.

The Rust-native workflow engine handles 10,000+ concurrent workflow executions with sub-second response. And because workflows are defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation.

**The immediate priority: define the Workflow entity with JSONB definition, implement basic workflow execution, and build the approval workflow. Workflows transform extracted data into business outcomes.**
